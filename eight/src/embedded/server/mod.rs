//! Server upgrades your storage into next level.

mod executor;
mod permission;

pub use permission::*;

use crate::{
    embedded::{
        language::QueryExecutor,
        messaging::{Request, Response},
        storage::Storage,
    },
    err,
};
use executor::Executor;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, oneshot, Mutex, RwLock},
    time,
};

struct ServerRequest {
    sender: oneshot::Sender<Response>,
    request: Request,
}

/// Server for Eight.
///
/// Server is based on [`Storage`] but makes it production grade.
///
/// Eight Server is focused on asynchronous execution. Every command spawns a new tokio task and messaging between command and requester done asynchronous.
/// Casts are just spawns commands and returns receiver channel so you can get the result later.
/// Calls are also wait for response. You can also add timeout for calls.
/// Server also has it is own redis-like query language.
#[derive(Clone)]
pub struct Server {
    executor: Arc<Executor>,
    sender: mpsc::UnboundedSender<ServerRequest>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<ServerRequest>>>,
    permission: Arc<RwLock<Permission>>,
}

impl Server {
    /// Creates new server from storage.
    ///
    /// ```no_run
    /// use eight::embedded::{server::Server, storage::filesystem::Storage};
    ///
    /// let storage = Storage::from_path("/path/to/store");
    /// let server = Server::new(storage);
    /// ```
    pub fn new(storage: impl Storage) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Self {
            executor: Arc::new(Executor::new(storage)),
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            permission: Default::default(),
        }
    }

    /// Set server permissions.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::embedded::{server::{Server, Permission}, storage::filesystem::Storage, messaging::{Request, Response}};
    ///
    /// let storage = Storage::from_path("./test_server_permission");
    /// let server = Server::new(storage);
    ///
    /// server.set_permission(Permission::Guest).await;
    /// server.start().await;
    ///
    /// let response = server.call(Request::Set("key".into(), "value".into())).await.unwrap();
    /// let Response::Error(_) = response else {
    ///   panic!("Must return an error");
    /// };
    ///
    /// server.set_permission(Permission::Owner).await;
    ///
    /// let response = server.call(Request::Set("key".into(), "value".into())).await.unwrap();
    /// let Response::Ok = response else {
    ///   panic!("Must return an error");
    /// };
    ///
    /// server.call(Request::Flush).await;
    /// # });
    /// ```
    pub async fn set_permission(&self, permission: Permission) {
        *self.permission.write().await = permission;
    }

    /// Run listener in another task so flow execution can continue.
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use eight::embedded::{server::Server, storage::memory::Storage};
    ///
    /// let storage = Storage::new();
    /// let server = Server::new(storage);
    ///
    /// server.start().await;
    /// # });
    /// ```
    pub async fn start(&self) {
        let server = self.clone();

        tokio::spawn(async move {
            server.listen().await;
        });
    }

    /// Run listener. This function blocks the flow.
    pub async fn listen(&self) {
        while let Some(request) = self.receiver.lock().await.recv().await {
            let ServerRequest { sender, request } = request;

            let executor = Arc::clone(&self.executor);
            let permission = Arc::clone(&self.permission);

            tokio::spawn(async move {
                let is_allowed = { permission.read().await.allowed(&request) };

                if let Err(error) = is_allowed {
                    sender.send(error.as_response()).ok();
                } else {
                    let response = match request {
                        Request::Set(key, value) => executor.set(key, value).await,
                        Request::Get(key) => executor.get(key).await,
                        Request::Delete(key) => executor.delete(key).await,
                        Request::Exists(key) => executor.exists(key).await,
                        Request::Increment(key, num) => executor.increment(key, num).await,
                        Request::Decrement(key, num) => executor.decrement(key, num).await,
                        Request::Search(key) => executor.search(key).await,
                        Request::Flush => executor.flush().await,
                        Request::DowngradePermission => {
                            let mut permission = permission.write().await;
                            *permission = permission.lower();

                            Response::Ok
                        }
                    };

                    sender.send(response).ok();
                }
            });
        }
    }

    /// Sends request to the server and returns response receiver. This function is useful when you need to run a command and get its result later.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::embedded::{server::Server, storage::memory::Storage, messaging::{Request, Response}};
    ///
    /// let storage = Storage::new();
    /// let server = Server::new(storage);
    ///
    /// server.start().await;
    ///
    /// let receiver = server.cast(Request::Exists("key".into())).await.unwrap();
    ///
    /// // ...
    ///
    /// assert_eq!(receiver.await.unwrap(), Response::Boolean(false));
    /// # });
    /// ```
    pub async fn cast(&self, request: Request) -> super::Result<oneshot::Receiver<Response>> {
        let (sender, receiver) = oneshot::channel();
        let request = ServerRequest { sender, request };

        if self.sender.send(request).is_err() {
            Err(err!(embedded, SendFail))
        } else {
            Ok(receiver)
        }
    }

    /// Sends request to the server and returns response.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::embedded::{server::Server, storage::memory::Storage, messaging::{Request, Response}};
    ///
    /// let storage = Storage::new();
    /// let server = Server::new(storage);
    ///
    /// server.start().await;
    ///
    /// let response = server.call(Request::Exists("key".into())).await.unwrap();
    ///
    /// assert_eq!(response, Response::Boolean(false));
    /// # });
    /// ```
    pub async fn call(&self, request: Request) -> super::Result<Response> {
        self.cast(request)
            .await?
            .await
            .map_err(|_| err!(embedded, RecvFail))
    }

    /// Same with call, but also takes a duration as a parameter which allows you to set a timeout for call.
    pub async fn call_in(&self, request: Request, timeout: Duration) -> super::Result<Response> {
        time::timeout(timeout, self.call(request))
            .await
            .map_err(|_| err!(embedded, RecvTimeout))?
    }

    /// Sends query to the server and returns response(s).
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::embedded::{server::Server, storage::memory::Storage, messaging::{Request, Response}};
    /// use std::collections::HashMap;
    ///
    /// let storage = Storage::new();
    /// let server = Server::new(storage);
    ///
    /// server.start().await;
    ///
    /// let mut env = HashMap::<String, String>::new();
    /// env.insert("user".into(), "pipi".into());
    /// env.insert("val".into(), "hello world!".into());
    ///
    /// let results = server.query("
    ///   set $user $val; # hello!
    ///   get $user;
    /// ", env).await.unwrap();
    ///
    /// assert_eq!(results.len(), 2);
    /// assert_eq!(results[0], Response::Ok);
    /// assert_eq!(results[1], Response::Text("hello world!".to_string()));
    /// # });
    /// ```
    pub async fn query<T>(
        &self,
        query: T,
        env: HashMap<String, String>,
    ) -> super::Result<Vec<Response>>
    where
        T: ToString,
    {
        let mut runtime = QueryExecutor::new(query.to_string(), env);
        runtime.execute(self).await
    }
}
