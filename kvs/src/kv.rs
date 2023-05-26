use crate::error::{ErrorKind, Result};
use crate::log::LogEntry;
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};
use std::path::PathBuf;

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
        };
        let position = store.read_all()?;
        store.writer.update_position(position)?;
        Ok(store)
    }

    /// Set the value of a string key to a string. Return an error if the value is not
    /// written successfully.
    pub fn set(&mut self, key: Key, value: String) -> Result<()> {
        let log_entry = LogEntry::add(key.clone(), value);
        let writing_start_position = self.writer.position;
        self.write_log(log_entry)?;
        let writing_end_position = self.writer.position;
        self.index.insert(
            key,
            Location {
                position: writing_start_position,
                length: writing_end_position - writing_start_position,
            },
        );
        Ok(())
    }

    /// Get the string value of a string key. If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
    pub fn get(&mut self, key: Key) -> Result<Option<String>> {
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
    pub fn remove(&mut self, key: Key) -> Result<()> {
        let log_entry = LogEntry::remove(key.clone());
        self.write_log(log_entry)?;
        self.index
            .remove(&key)
            .ok_or(ErrorKind::KeyNotFound)
            .map(|_| ())
    }

    fn write_log(&mut self, log_entry: LogEntry) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &log_entry)?;
        self.writer.flush()?;
        Ok(())
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
            } else {
                self.index.insert(
                    log_entry.key,
                    Location {
                        position: current_pos,
                        length: next_pos - current_pos,
                    },
                );
            }
            current_pos = next_pos;
        }
        Ok(current_pos)
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
