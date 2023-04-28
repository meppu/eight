//! Client implementation for WebSocket connections.

use super::messaging;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    net::TcpStream,
    sync::{oneshot, Mutex},
};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

type WebSocketConnection = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// WebSocket client struct.
#[derive(Clone)]
pub struct Client {
    sender: Arc<Mutex<SplitSink<WebSocketConnection, Message>>>,
    receiver: Arc<Mutex<SplitStream<WebSocketConnection>>>,
    pool: Arc<Mutex<HashMap<String, oneshot::Sender<messaging::Response>>>>,
}

impl Client {
    /// Create WebSocket client and start connection.
    ///
    /// ```no_run
    /// # async fn howdy() {
    /// use eight::client::websocket::Client;
    ///
    /// let client = Client::connect("http://localhost:3000/").await;
    /// # }
    /// ```
    pub async fn connect(host: &str) -> super::Result<Self> {
        let (connection, _) = connect_async(format!("{host}/rpc"))
            .await
            .map_err(|_| super::Error::WebSocketConnectionFail)?;

        let (sender, receiver) = connection.split();

        Ok(Self {
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
            pool: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Run listener in another task.
    pub async fn start(&self) {
        let clone = self.clone();

        tokio::spawn(async move {
            clone.listen().await;
        });
    }

    /// Message broker for WebSocket connection. Distributes responses through channels by request id. This function blocks the flow.
    pub async fn listen(&self) {
        while let Ok(message) = self.receiver.lock().await.try_next().await {
            if let Some(Message::Text(message)) = message {
                let pool = Arc::clone(&self.pool);

                tokio::spawn(async move {
                    let Ok(decoded) = serde_json::from_str::<messaging::Response>(&message) else {
                        return;
                    };

                    if let Some(sender) = pool.lock().await.remove(&decoded.id) {
                        sender.send(decoded).ok();
                    }
                });
            }
        }
    }

    /// Execute query asynchronous. Returns an oneshot receiver so you can manually receive response later.
    ///
    /// ```no_run
    /// # async fn howdy2() {
    /// use eight::client::{messaging::QueryBuilder, websocket::Client};
    ///
    /// let client = Client::connect("http://localhost:3000/").await.unwrap();
    /// client.start().await;
    ///
    /// let request = QueryBuilder::new()
    ///   .add_query("set $user $value;")
    ///   .bind("user", "bob")
    ///   .bind("value", "some random data")
    ///   .set_random_id()
    ///   .collect();
    ///
    /// let resp_recv = client.cast(request).await;
    /// # }
    /// ```
    pub async fn cast(
        &self,
        request: messaging::Request,
    ) -> super::Result<oneshot::Receiver<messaging::Response>> {
        let (sender, receiver) = oneshot::channel::<messaging::Response>();
        self.pool.lock().await.insert(request.id.clone(), sender);

        let raw_request = serde_json::to_string(&request).unwrap_or_default();

        let result = self
            .sender
            .lock()
            .await
            .send(Message::Text(raw_request.into()))
            .await;

        if result.is_err() {
            Err(super::Error::WebSocketSendFail)
        } else {
            Ok(receiver)
        }
    }

    /// Execute query and wait for response.
    ///
    /// ```no_run
    /// # async fn howdy3() {
    /// use eight::client::{messaging::QueryBuilder, websocket::Client};
    ///
    /// let client = Client::connect("http://localhost:3000/").await.unwrap();
    /// client.start().await;
    ///
    /// let request = QueryBuilder::new()
    ///   .add_query("get $user;")
    ///   .bind("user", "bob")
    ///   .set_random_id()
    ///   .collect();
    ///
    /// let resp = client.call(request).await;
    /// # }
    /// ```
    pub async fn call(&self, request: messaging::Request) -> super::Result<messaging::Response> {
        self.cast(request)
            .await?
            .await
            .map_err(|_| super::Error::WebSocketReceiveFail)
    }
}
