use crate::embedded::{messaging::Response, Storage};

pub(super) struct Executor {
    storage: Storage,
}

impl Executor {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub async fn set(&self, key: String, value: String) -> Response {
        match self.storage.set(key, value).await {
            Ok(_) => Response::Ok,
            Err(error) => error.as_response(),
        }
    }

    pub async fn get(&self, key: String) -> Response {
        match self.storage.get(key).await {
            Ok(value) => Response::Text(value),
            Err(error) => error.as_response(),
        }
    }

    pub async fn delete(&self, key: String) -> Response {
        match self.storage.delete(key).await {
            Ok(_) => Response::Ok,
            Err(error) => error.as_response(),
        }
    }

    pub async fn exists(&self, key: String) -> Response {
        match self.storage.exists(key).await {
            Ok(value) => Response::Boolean(value),
            Err(error) => error.as_response(),
        }
    }

    pub async fn increment(&self, key: String, value: usize) -> Response {
        match self.storage.increment(key, value).await {
            Ok(new) => Response::Number(new),
            Err(error) => error.as_response(),
        }
    }

    pub async fn decrement(&self, key: String, value: usize) -> Response {
        match self.storage.decrement(key, value).await {
            Ok(new) => Response::Number(new),
            Err(error) => error.as_response(),
        }
    }

    pub async fn search(&self, key: String) -> Response {
        match self.storage.search(key).await {
            Ok(value) => Response::TextList(value),
            Err(error) => error.as_response(),
        }
    }

    pub async fn flush(&self) -> Response {
        match self.storage.flush().await {
            Ok(_) => Response::Ok,
            Err(error) => error.as_response(),
        }
    }
}
