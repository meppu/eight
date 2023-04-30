//! Official in-memory storage implementation for eight.

use crate::{embedded, err};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// In-memory storage. Preferred for temporary key-values (like cache).
#[derive(Debug, Default)]
pub struct Storage {
    values: RwLock<HashMap<String, String>>,
}

impl Storage {
    /// Create new in-memory storage.
    ///
    /// This function is same with [`Default::default`].
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl super::Storage for Storage {
    async fn set(&self, key: String, value: String) -> embedded::Result<()> {
        self.values.write().await.insert(key, value);
        Ok(())
    }

    async fn get(&self, key: String) -> embedded::Result<String> {
        if let Some(value) = self.values.read().await.get(&key) {
            Ok(value.to_owned())
        } else {
            Err(err!(embedded, GetKeyFail))
        }
    }

    async fn delete(&self, key: String) -> embedded::Result<()> {
        if self.values.write().await.remove(&key).is_some() {
            Ok(())
        } else {
            Err(err!(embedded, DeleteKeyFail))
        }
    }

    async fn exists(&self, key: String) -> embedded::Result<bool> {
        Ok(self.values.read().await.get(&key).is_some())
    }

    async fn increment(&self, key: String, num: usize) -> embedded::Result<usize> {
        let raw = self.get(key.clone()).await?;
        let new = raw
            .parse::<usize>()
            .map_err(|_| err!(embedded, UIntParseFail))?
            + num;

        self.set(key, new.to_string()).await?;
        Ok(new)
    }

    async fn decrement(&self, key: String, num: usize) -> embedded::Result<usize> {
        let raw = self.get(key.clone()).await?;
        let new = raw
            .parse::<usize>()
            .map_err(|_| err!(embedded, UIntParseFail))?
            - num;

        self.set(key, new.to_string()).await?;
        Ok(new)
    }

    async fn search(&self, key: String) -> embedded::Result<Vec<String>> {
        Ok(self
            .values
            .read()
            .await
            .keys()
            .filter(|&x| x.starts_with(&key))
            .cloned()
            .collect::<Vec<_>>())
    }

    async fn flush(&self) -> embedded::Result<()> {
        self.values.write().await.clear();
        Ok(())
    }
}
