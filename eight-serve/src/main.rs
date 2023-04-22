use axum::{
    routing::{get, post},
    Router,
};
use eight::Server;
use std::{net::SocketAddr, str::FromStr};

mod http;
mod query;

#[tokio::main]
async fn main() {
    let server = Server::from_str("./test").unwrap();
    server.start().await;

    let app = Router::new()
        .route("/", get(root))
        .route("/query", post(http::run_query))
        .with_state(server);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
