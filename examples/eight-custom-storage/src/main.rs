use std::collections::{HashMap, VecDeque};

use eight::{
    self,
    embedded::{
        self,
        messaging::Response,
        server::Server,
        storage::{async_trait, Storage},
    },
    env, request,
};
use tokio::sync::RwLock;

struct FifoCache {
    order: RwLock<VecDeque<String>>,
    values: RwLock<HashMap<String, String>>,
    max: usize,
}

impl FifoCache {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            order: RwLock::new(VecDeque::new()),
            values: RwLock::new(HashMap::new()),
        }
    }

    pub async fn pop(&self) {
        while let Some(key) = self.order.write().await.pop_front() {
            if self.values.write().await.remove(&key).is_some() {
                break;
            }
        }
    }
}

#[async_trait]
impl Storage for FifoCache {
    async fn set(&self, key: String, value: String) -> embedded::Result<()> {
        if self.values.read().await.len() == self.max {
            self.pop().await;
        }

        self.values.write().await.insert(key.clone(), value);
        self.order.write().await.push_back(key);

        Ok(())
    }

    async fn get(&self, key: String) -> embedded::Result<String> {
        match self.values.read().await.get(&key) {
            Some(value) => Ok(value.clone()),
            None => Err(embedded::Error::GetKeyFail),
        }
    }

    async fn delete(&self, key: String) -> embedded::Result<()> {
        match self.values.write().await.remove(&key) {
            Some(_) => Ok(()),
            None => Err(embedded::Error::DeleteKeyFail),
        }
    }

    async fn exists(&self, key: String) -> embedded::Result<bool> {
        Ok(self.get(key).await.is_ok())
    }

    async fn increment(&self, key: String, num: usize) -> embedded::Result<usize> {
        match self.values.write().await.get_mut(&key) {
            Some(value) => {
                if let Ok(number) = value.parse::<usize>() {
                    let result = number + num;

                    *value = result.to_string();
                    Ok(result)
                } else {
                    Err(embedded::Error::UIntParseFail)
                }
            }
            None => Err(embedded::Error::GetKeyFail),
        }
    }

    async fn decrement(&self, key: String, num: usize) -> embedded::Result<usize> {
        match self.values.write().await.get_mut(&key) {
            Some(value) => {
                if let Ok(number) = value.parse::<usize>() {
                    let result = number - num;

                    *value = result.to_string();
                    Ok(result)
                } else {
                    Err(embedded::Error::UIntParseFail)
                }
            }
            None => Err(embedded::Error::GetKeyFail),
        }
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
        self.order.write().await.clear();

        Ok(())
    }
}

#[tokio::main]
async fn main() -> embedded::Result<()> {
    let storage = FifoCache::new(3);
    let server = Server::new(storage);

    server.start().await;

    server
        .query("set $test $value;", env!(test: "example", value: 10))
        .await?;

    server.query("set $test 20;", env!(test: "second")).await?;
    server.query("set a b; set b a;", env!()).await?;

    assert_eq!(
        server.call(request!(Get, "example")).await?,
        Response::Error(embedded::Error::GetKeyFail)
    );

    assert_eq!(
        server.call(request!(Get, "second")).await?,
        Response::Text("20".into())
    );

    server
        .query("incr $test $val;", env!(test: "second", val: 10))
        .await?;

    assert_eq!(
        server.call(request!(Get, "second")).await?,
        Response::Text("30".into())
    );

    Ok(())
}
