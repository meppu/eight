use crate::{
    embedded::{self, filesystem},
    err,
};
use async_trait::async_trait;
use std::{path::PathBuf, str::FromStr};

/// Filesystem based storage. Preferred when you need to keep key-values on disk.
#[derive(Debug, Default)]
pub struct FileStorage {
    path: PathBuf,
}

impl FromStr for FileStorage {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            path: PathBuf::from_str(s)?,
        })
    }
}

impl FileStorage {
    /// Create new file-storage.
    ///
    /// This function is same with [`Default::default`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Create new file-storage from path.
    ///
    /// ```no_run
    /// use eight::embedded::FileStorage;
    ///
    /// FileStorage::from_path("/tmp/test");
    /// ```
    pub fn from_path<T>(path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        Self { path: path.into() }
    }
}

#[async_trait]
impl super::Storage for FileStorage {
    async fn set(&self, key: String, value: String) -> embedded::Result<()> {
        let mut path = filesystem::create_path(&self.path, &key)?;
        filesystem::write(&mut path, value).await
    }

    async fn get(&self, key: String) -> embedded::Result<String> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::read(&path).await
    }

    async fn delete(&self, key: String) -> embedded::Result<()> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::delete(&path).await
    }

    async fn exists(&self, key: String) -> embedded::Result<bool> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::exists(&path).await
    }

    async fn increment(&self, key: String, num: usize) -> embedded::Result<usize> {
        let mut path = filesystem::create_path(&self.path, &key)?;

        let raw = filesystem::read(&path).await?;
        let new = raw
            .parse::<usize>()
            .map_err(|_| err!(embedded, UIntParseFail))?
            + num;

        filesystem::write(&mut path, new.to_string()).await?;
        Ok(new)
    }

    async fn decrement(&self, key: String, num: usize) -> embedded::Result<usize> {
        let mut path = filesystem::create_path(&self.path, &key)?;

        let raw = filesystem::read(&path).await?;
        let new = raw
            .parse::<usize>()
            .map_err(|_| err!(embedded, UIntParseFail))?
            - num;

        filesystem::write(&mut path, new.to_string()).await?;
        Ok(new)
    }

    async fn search(&self, key: String) -> embedded::Result<Vec<String>> {
        filesystem::search(&self.path, &key).await
    }

    async fn flush(&self) -> embedded::Result<()> {
        filesystem::flush(&self.path).await
    }
}
