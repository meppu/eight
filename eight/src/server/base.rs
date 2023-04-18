use super::{executor::Executor, message::ServerRequest};
use crate::{language::QueryExecutor, EightError, EightResult, Request, Response, Storage};
use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, oneshot, Mutex},
    time,
};

#[derive(Debug, Clone)]
pub struct Server {
    storage: Arc<Storage>,
    sender: mpsc::UnboundedSender<ServerRequest>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<ServerRequest>>>,
}

impl FromStr for Server {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let storage = Storage::from_str(s)?;
        Ok(Server::new(storage))
    }
}

impl Server {
    pub fn new(storage: Storage) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Self {
            storage: Arc::new(storage),
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub async fn start(&self) {
        let server = self.clone();

        tokio::spawn(async move {
            server.listen().await;
        });
    }

    pub async fn listen(&self) {
        while let Some(request) = self.receiver.lock().await.recv().await {
            let ServerRequest { sender, request } = request;
            let storage = Arc::clone(&self.storage);

            tokio::spawn(async move {
                let response = match request {
                    Request::Set(key, value) => Executor::set(storage, key, value).await,
                    Request::Get(key) => Executor::get(storage, key).await,
                    Request::Delete(key) => Executor::delete(storage, key).await,
                    Request::Exists(key) => Executor::exists(storage, key).await,
                    Request::Increment(key, num) => Executor::increment(storage, key, num).await,
                    Request::Decrement(key, num) => Executor::decrement(storage, key, num).await,
                    Request::Flush => Executor::flush(storage).await,
                };

                sender.send(response).ok();
            });
        }
    }

    pub async fn cast(&self, request: Request) -> EightResult<oneshot::Receiver<Response>> {
        let (sender, receiver) = oneshot::channel();
        let request = ServerRequest { sender, request };

        if self.sender.send(request).is_err() {
            Err(EightError::SendFail)
        } else {
            Ok(receiver)
        }
    }

    pub async fn call(&self, request: Request) -> EightResult<Response> {
        if let Ok(value) = self.cast(request).await?.await {
            Ok(value)
        } else {
            Err(EightError::RecvFail)
        }
    }

    pub async fn call_in(&self, request: Request, timeout: Duration) -> EightResult<Response> {
        if let Ok(value) = time::timeout(timeout, self.call(request)).await {
            value
        } else {
            Err(EightError::RecvTimeout)
        }
    }

    pub async fn query<T>(
        &self,
        query: T,
        env: HashMap<String, String>,
    ) -> EightResult<Vec<Response>>
    where
        T: ToString,
    {
        let mut runtime = QueryExecutor::new(query.to_string(), env);
        runtime.execute(self).await
    }
}
