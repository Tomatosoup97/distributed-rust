use super::{Engine, KvsEngine};
use crate::error::{ErrorKind, Result};

/// Sled engine wrapper
pub struct SledKvsEngine {
    db: sled::Db,
}

impl SledKvsEngine {
    /// Create a new SledKvsEngine
    pub fn new(db: sled::Db) -> Self {
        SledKvsEngine { db }
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key, value.into_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self
            .db
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.db.remove(key)?.ok_or(ErrorKind::KeyNotFound)?;
        self.db.flush()?;
        Ok(())
    }

    fn as_type(&self) -> Engine {
        Engine::sled
    }
}
