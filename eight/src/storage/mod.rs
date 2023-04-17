use crate::{filesystem, EightError, EightResult};
use std::{path::PathBuf, str::FromStr};

#[cfg(test)]
mod tests;

#[derive(Debug, Default, Clone)]
pub struct Storage {
    path: PathBuf,
}

impl FromStr for Storage {
    type Err = core::convert::Infallible;

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

    pub async fn set(&self, key: String, value: String) -> EightResult<()> {
        let mut path = filesystem::create_path(&self.path, &key)?;
        filesystem::write(&mut path, value).await
    }

    pub async fn get(&self, key: String) -> EightResult<String> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::read(&path).await
    }

    pub async fn delete(&self, key: String) -> EightResult<()> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::delete(&path).await
    }

    pub async fn increment(&self, key: String, add: usize) -> EightResult<usize> {
        let mut path = filesystem::create_path(&self.path, &key)?;

        let raw = filesystem::read(&path).await?;

        if let Ok(value) = raw.parse::<usize>() {
            filesystem::write(&mut path, (value + add).to_string()).await?;
            Ok(value)
        } else {
            Err(EightError::UIntParseFail)
        }
    }

    pub async fn decrement(&self, key: String, add: usize) -> EightResult<usize> {
        let mut path = filesystem::create_path(&self.path, &key)?;

        let raw = filesystem::read(&path).await?;

        if let Ok(value) = raw.parse::<usize>() {
            filesystem::write(&mut path, (value - add).to_string()).await?;
            Ok(value)
        } else {
            Err(EightError::UIntParseFail)
        }
    }

    pub async fn exists(&self, key: String) -> EightResult<bool> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::exists(&path).await
    }

    pub async fn flush(&self) -> EightResult<()> {
        filesystem::flush(&self.path).await
    }
}
