use crate::{Response, Storage};

use std::sync::Arc;

pub(super) struct Executor;

impl Executor {
    pub async fn set(storage: Arc<Storage>, key: String, value: String) -> Response {
        match storage.set(key, value).await {
            Ok(_) => Response::Ok,
            _ => Response::Error(
                "Write operation is failed due to permission reasons or an invalid key",
            ),
        }
    }

    pub async fn get(storage: Arc<Storage>, key: String) -> Response {
        match storage.get(key).await {
            Ok(value) => Response::Value(value),
            _ => Response::Error("Key doesn't exists"),
        }
    }

    pub async fn delete(storage: Arc<Storage>, key: String) -> Response {
        match storage.delete(key).await {
            Ok(_) => Response::Ok,
            _ => Response::Error("Key doesn't exists"),
        }
    }

    pub async fn exists(storage: Arc<Storage>, key: String) -> Response {
        match storage.exists(key).await {
            Ok(true) => Response::Ok,
            _ => Response::Error("Key doesn't exists"),
        }
    }

    pub async fn increment(storage: Arc<Storage>, key: String, value: usize) -> Response {
        match storage.increment(key, value).await {
            Ok(new) => Response::Value(new.to_string()),
            _ => Response::Error("Failed to increment, key doesn't exists or not an integer"),
        }
    }

    pub async fn decrement(storage: Arc<Storage>, key: String, value: usize) -> Response {
        match storage.decrement(key, value).await {
            Ok(new) => Response::Value(new.to_string()),
            _ => Response::Error("Failed to decrement, key doesn't exists or not an integer"),
        }
    }

    pub async fn flush(storage: Arc<Storage>) -> Response {
        match storage.flush().await {
            Ok(_) => Response::Ok,
            _ => Response::Error("Failed to flush"),
        }
    }
}
