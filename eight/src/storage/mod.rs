mod interface;

use crate::filesystem;
use anyhow::Result;
pub use interface::Storage;

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
