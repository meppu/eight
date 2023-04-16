use super::{executor::Executor, message::ServerRequest};
use crate::{Request, Response, Storage};

use std::{sync::Arc, time::Duration};
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

                let _ = sender.send(response);
            });
        }
    }

    pub async fn cast(&self, request: Request) -> anyhow::Result<oneshot::Receiver<Response>> {
        let (sender, receiver) = oneshot::channel();
        let request = ServerRequest { sender, request };

        self.sender.send(request)?;

        Ok(receiver)
    }

    pub async fn call(&self, request: Request) -> anyhow::Result<Response> {
        Ok(self.cast(request).await?.await?)
    }

    pub async fn call_in(&self, request: Request, timeout: Duration) -> anyhow::Result<Response> {
        time::timeout(timeout, self.call(request)).await?
    }
}
