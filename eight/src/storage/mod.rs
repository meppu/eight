use crate::filesystem;

use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Default)]
pub struct Storage {
    path: PathBuf,
}

impl FromStr for Storage {
    type Err = anyhow::Error;

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

    pub async fn set(&self, key: String, value: String) -> anyhow::Result<()> {
        let mut path = filesystem::create_path(&self.path, &key)?;
        filesystem::write(&mut path, value).await
    }

    pub async fn get(&self, key: String) -> anyhow::Result<String> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::read(&path).await
    }

    pub async fn delete(&self, key: String) -> anyhow::Result<()> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::delete(&path).await
    }

    pub async fn increment(&self, key: String, add: usize) -> anyhow::Result<usize> {
        let mut path = filesystem::create_path(&self.path, &key)?;

        let raw = filesystem::read(&path).await?;
        let value = raw.parse::<usize>()? + add;

        filesystem::write(&mut path, value.to_string()).await?;

        Ok(value)
    }

    pub async fn decrement(&self, key: String, add: usize) -> anyhow::Result<usize> {
        let mut path = filesystem::create_path(&self.path, &key)?;

        let raw = filesystem::read(&path).await?;
        let value = raw.parse::<usize>()? - add;

        filesystem::write(&mut path, value.to_string()).await?;

        Ok(value)
    }

    pub async fn exists(&self, key: String) -> anyhow::Result<bool> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::exists(&path).await
    }

    pub async fn flush(&self) -> anyhow::Result<()> {
        filesystem::flush(&self.path).await
    }
}
