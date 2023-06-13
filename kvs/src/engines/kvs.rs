use super::{Engine, KvsEngine};
use crate::error::{ErrorKind, Result};
use crate::log::LogEntry;
use serde_json::Deserializer;
use slog_scope::info;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Seek, SeekFrom};
use std::path::PathBuf;

const COMPACTNESS_THRESHOLD: u64 = 1024;

type Position = u64;

#[derive(Debug)]
struct Location {
    position: Position,
    length: u64,
}
type Key = String;

/// A key-value store.
#[derive(Debug)]
pub struct KvStore {
    writer: BufWriterWithPos<File>,
    reader: BufReader<File>,
    index: HashMap<String, Location>,
    to_compact: u64,
    dir: PathBuf,
}

impl KvsEngine for KvStore {
    /// Set the value of a string key to a string. Return an error if the value is not
    /// written successfully.
    fn set(&mut self, key: Key, value: String) -> Result<()> {
        let log_entry = LogEntry::add(key.clone(), value);
        let writing_start_position = self.writer.position;
        serde_json::to_writer(&mut self.writer, &log_entry)?;
        self.writer.flush()?;
        let writing_end_position = self.writer.position;
        if self
            .index
            .insert(
                key,
                Location {
                    position: writing_start_position,
                    length: writing_end_position - writing_start_position,
                },
            )
            .is_some()
        {
            self.to_compact += 1;
        }
        if self.to_compact > COMPACTNESS_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    /// Get the string value of a string key. If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
    fn get(&mut self, key: Key) -> Result<Option<String>> {
        match self.index.get(&key) {
            Some(&Location { position, length }) => {
                self.reader.seek(SeekFrom::Start(position))?;
                let length_bound_reader = self.reader.get_mut().take(length);
                let log_entry: LogEntry = serde_json::from_reader(length_bound_reader)?;
                Ok(Some(log_entry.value))
            }
            None => Ok(None),
        }
    }

    /// Remove a given key. Return an error if the key does not exist or is not removed
    /// successfully.
    fn remove(&mut self, key: Key) -> Result<()> {
        let log_entry = LogEntry::remove(key.clone());
        serde_json::to_writer(&mut self.writer, &log_entry)?;
        self.writer.flush()?;
        self.to_compact += 1;
        self.index
            .remove(&key)
            .ok_or(ErrorKind::KeyNotFound)
            .map(|_| ())?;

        if self.to_compact > COMPACTNESS_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    fn as_type(&self) -> Engine {
        Engine::kvs
    }
}

impl KvStore {
    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        let log_path = path.join("data.log");

        let writer_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&log_path)?;
        let reader_file = OpenOptions::new().read(true).open(&log_path)?;

        let mut store = KvStore {
            writer: BufWriterWithPos::new(writer_file)?,
            reader: BufReader::new(reader_file),
            index: HashMap::new(),
            dir: path,
            to_compact: 0,
        };
        let position = store.read_all()?;
        store.writer.update_position(position)?;

        if store.to_compact > COMPACTNESS_THRESHOLD {
            store.compact()?;
        }
        Ok(store)
    }

    fn read_all(&mut self) -> Result<Position> {
        // To make sure we read from the beginning of the file
        let mut current_pos = self.reader.seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(&mut self.reader).into_iter::<LogEntry>();

        while let Some(log_entry) = stream.next() {
            let next_pos = stream.byte_offset() as u64;

            let log_entry = log_entry?;
            if log_entry.is_tombstone() {
                self.index.remove(&log_entry.key);
                self.to_compact += 1;
            } else if self
                .index
                .insert(
                    log_entry.key,
                    Location {
                        position: current_pos,
                        length: next_pos - current_pos,
                    },
                )
                .is_some()
            {
                self.to_compact += 1;
            }
            current_pos = next_pos;
        }
        Ok(current_pos)
    }

    fn compact(&mut self) -> Result<()> {
        /* Compaction algorithm. The current strategy is to create a new file and copy
         * over all the entries in the index to the new file. Then, we replace the old file
         * with the new file and update the index.
         */
        info!("Compacting...");
        let compacted_log_path = self.dir.join("data--compacted.log");
        let original_log_path = self.dir.join("data.log");
        let writer_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&compacted_log_path)?;
        let mut compaction_writer = BufWriterWithPos::new(writer_file)?;

        let mut new_position = 0;

        for (key, location) in self.index.iter_mut() {
            self.reader.seek(SeekFrom::Start(location.position))?;
            let mut length_bound_reader = self.reader.get_mut().take(location.length);
            io::copy(&mut length_bound_reader, &mut compaction_writer)?;

            *location = Location {
                position: new_position,
                length: location.length,
            };
            new_position += location.length;
        }

        fs::rename(&compacted_log_path, &original_log_path)?;
        self.writer = compaction_writer;
        self.to_compact = 0;
        self.reader = BufReader::new(OpenOptions::new().read(true).open(&original_log_path)?);
        self.writer.update_position(new_position)?;

        Ok(())
    }
}

#[derive(Debug)]
struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    position: Position,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let position = inner.stream_position()?;

        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            position,
        })
    }

    fn update_position(&mut self, position: Position) -> Result<()> {
        self.position = position;
        self.writer.seek(SeekFrom::Start(position))?;
        Ok(())
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.position += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.position = self.writer.seek(pos)?;
        Ok(self.position)
    }
}
