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
type Key = String;

/// A key-value store.
#[derive(Debug)]
pub struct KvStore {
    writer: BufWriter<File>,
    reader: BufReader<File>,
    index: HashMap<String, Position>,
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
            writer: BufWriter::new(writer_file),
            reader: BufReader::new(reader_file),
            index: HashMap::new(),
        };
        store.read_all()?;
        // println!("store: {:?}", store.map);
        Ok(store)
    }

    /// Set the value of a string key to a string. Return an error if the value is not
    /// written successfully.
    pub fn set(&mut self, key: Key, value: String) -> Result<()> {
        let log_entry = LogEntry::add(key.clone(), value.clone());
        self.write(log_entry)?;
        self.map.insert(key, value);
        Ok(())
    }

    /// Get the string value of a string key. If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
    pub fn get(&mut self, key: Key) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    /// Remove a given key. Return an error if the key does not exist or is not removed
    /// successfully.
    pub fn remove(&mut self, key: Key) -> Result<()> {
        let log_entry = LogEntry::remove(key.clone());
        self.write(log_entry)?;
        self.index
            .remove(&key)
            .ok_or(ErrorKind::KeyNotFound)
            .map(|_| ())
    }

    fn write(&mut self, log_entry: LogEntry) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &log_entry)?;
        self.writer.flush()?;
        Ok(())
    }

    fn read_all(&mut self) -> Result<()> {
        // To make sure we read from the beginning of the file
        let mut current_pos = self.reader.seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(&mut self.reader).into_iter::<LogEntry>();

        while let Some(log_entry) = stream.next() {
            let next_pos = stream.byte_offset() as u64;

            let log_entry = log_entry?;
            if log_entry.is_tombstone() {
                self.index.remove(&log_entry.key);
            } else {
                self.index.insert(log_entry.key, current_pos);
            }
            current_pos = next_pos;
        }
        Ok(())
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}
