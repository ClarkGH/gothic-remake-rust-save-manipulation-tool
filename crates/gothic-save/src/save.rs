use std::path::Path;

use crate::{Error, Result};

#[derive(Debug)]
pub struct Save {
    // fields we discover from reverse-engineering
    pub raw: Vec<u8>,
}

impl Save {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let raw = std::fs::read(path)?;
        // TODO: validate header, parse sections
        Ok(Self { raw })
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        std::fs::write(path, &self.raw)?;
        Ok(())
    }
}
