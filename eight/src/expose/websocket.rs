use crate::client::messaging::{Request, Response};
use crate::embedded::Server;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
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
    socket.on_upgrade(move |socket| execute_loop(database, socket))
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
    let Ok(payload) = serde_json::from_str::<Request>(&raw_value) else {
        return;
    };

    let Request { query, vars, id } = payload;
    let response = database.query(query, vars).await;

    let response = match response {
        Ok(results) => Response { id, results },
        Err(error) => Response {
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
