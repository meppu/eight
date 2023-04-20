use crate::{Response, Storage};
use std::sync::Arc;

pub(super) struct Executor;

impl Executor {
    pub async fn set(storage: Arc<Storage>, key: String, value: String) -> Response {
        match storage.set(key, value).await {
            Ok(_) => Response::Ok,
            Err(error) => error.as_response(),
        }
    }

    pub async fn get(storage: Arc<Storage>, key: String) -> Response {
        match storage.get(key).await {
            Ok(value) => Response::Text(value),
            Err(error) => error.as_response(),
        }
    }

    pub async fn delete(storage: Arc<Storage>, key: String) -> Response {
        match storage.delete(key).await {
            Ok(_) => Response::Ok,
            Err(error) => error.as_response(),
        }
    }

    pub async fn exists(storage: Arc<Storage>, key: String) -> Response {
        match storage.exists(key).await {
            Ok(value) => Response::Boolean(value),
            Err(error) => error.as_response(),
        }
    }

    pub async fn increment(storage: Arc<Storage>, key: String, value: usize) -> Response {
        match storage.increment(key, value).await {
            Ok(new) => Response::Number(new),
            Err(error) => error.as_response(),
        }
    }

    pub async fn decrement(storage: Arc<Storage>, key: String, value: usize) -> Response {
        match storage.decrement(key, value).await {
            Ok(new) => Response::Number(new),
            Err(error) => error.as_response(),
        }
    }

    pub async fn flush(storage: Arc<Storage>) -> Response {
        match storage.flush().await {
            Ok(_) => Response::Ok,
            Err(error) => error.as_response(),
        }
    }
}
