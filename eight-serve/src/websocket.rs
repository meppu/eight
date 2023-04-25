use crate::query::{QueryRequest, QueryResponse};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use eight::Server;
use futures::{
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_connection(
    State(database): State<Server>,
    socket: WebSocketUpgrade,
) -> impl IntoResponse {
    let database = Arc::new(database);
    socket.on_upgrade(move |socket| receive_spawner(database, socket))
}

async fn receive_spawner(database: Arc<Server>, mut socket: WebSocket) {
    if socket.send(Message::Ping(vec![9, 6])).await.is_err() {
        return;
    };

    execute_loop(database, socket).await;
}

async fn execute_loop(database: Arc<Server>, socket: WebSocket) {
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(Mutex::new(sender));

    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(raw_value) => {
                let sender = Arc::clone(&sender);
                let database = Arc::clone(&database);

                tokio::spawn(message_process(database, sender, raw_value));
            }
            _ => {}
        }
    }
}

async fn message_process(
    database: Arc<Server>,
    sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    raw_value: String,
) {
    let Ok(payload) = serde_json::from_str::<QueryRequest>(&raw_value) else {
        sender.lock().await.send(Message::Text(r#"{"error": "Parsing request failed"}"#.to_string())).await.ok();
        return;
    };

    let QueryRequest { query, vars, id } = payload;
    let response = database.query(query, vars).await;

    let response = match response {
        Ok(results) => QueryResponse { id, results },
        Err(error) => QueryResponse {
            id,
            results: vec![error.as_response()],
        },
    };

    let raw_response = serde_json::to_string(&response).unwrap_or_default();

    sender
        .lock()
        .await
        .send(Message::Text(raw_response))
        .await
        .ok();
}
