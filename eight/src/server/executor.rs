use anyhow::anyhow;

use crate::{Response, Storage};

use std::sync::Arc;

pub(super) struct Executor;

impl Executor {
    pub async fn set(storage: Arc<Storage>, key: String, value: String) -> Response {
        match storage.set(key, value).await {
            Ok(_) => Response::Ok,
            _ => Response::Error(anyhow!("Write operation is failed")),
        }
    }

    pub async fn get(storage: Arc<Storage>, key: String) -> Response {
        match storage.get(key).await {
            Ok(value) => Response::Value(value),
            _ => Response::Error(anyhow!("Key doesn't exists")),
        }
    }

    pub async fn delete(storage: Arc<Storage>, key: String) -> Response {
        match storage.delete(key).await {
            Ok(_) => Response::Ok,
            _ => Response::Error(anyhow!("Key doesn't exists")),
        }
    }

    pub async fn exists(storage: Arc<Storage>, key: String) -> Response {
        match storage.exists(key).await {
            Ok(true) => Response::Ok,
            _ => Response::Error(anyhow!("Key doesn't exists")),
        }
    }

    pub async fn flush(storage: Arc<Storage>) -> Response {
        match storage.flush().await {
            Ok(_) => Response::Ok,
            _ => Response::Error(anyhow!("Failed to flush")),
        }
    }
}
