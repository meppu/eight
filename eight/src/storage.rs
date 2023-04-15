use anyhow::{Error, Result};
use std::{path::PathBuf, str::FromStr};

use crate::filesystem;

#[derive(Debug, Default)]
pub struct Storage {
    path: PathBuf,
}

impl FromStr for Storage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            path: PathBuf::from_str(s)?,
            ..Default::default()
        })
    }
}

impl Storage {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn set(&self, key: String, value: String) -> Result<()> {
        let mut path = filesystem::create_path(&self.path, &key)?;
        filesystem::write(&mut path, value).await
    }

    pub async fn get(&self, key: String) -> Result<String> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::read(&path).await
    }

    pub async fn delete(&self, key: String) -> Result<()> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::delete(&path).await
    }

    pub async fn flush(&self) -> Result<()> {
        filesystem::flush(&self.path).await
    }
}
