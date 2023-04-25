use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use eight::{Permission, Server};
use std::{net::SocketAddr, str::FromStr};

mod cli;
mod http;
mod query;
mod websocket;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let args = cli::Args::parse();

    let server = Server::from_str(&args.directory).unwrap();
    server.start().await;

    match args.permission {
        0 => server.set_permission(Permission::Guest).await,
        1 => server.set_permission(Permission::Admin).await,
        2 => server.set_permission(Permission::Owner).await,
        _ => return Err("Invalid permission value. For more information, try '--help'."),
    }

    let app = Router::new()
        .route("/", get(root))
        .route("/query", post(http::run_query))
        .route("/rpc", get(websocket::handle_connection))
        .with_state(server);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
