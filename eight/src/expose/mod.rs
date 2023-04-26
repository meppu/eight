mod http;
mod websocket;

use crate::embedded::{Permission, Server};
use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Config {
    addr: SocketAddr,
    fallback_path: Option<String>,
    permission: Permission,
    server: Server,
}

#[derive(Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn from_server(server: Server) -> Self {
        Self {
            config: Config {
                addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
                fallback_path: None,
                permission: Default::default(),
                server,
            },
        }
    }

    pub fn bind(mut self, addr: SocketAddr) -> Self {
        self.config.addr = addr;
        self
    }

    pub fn set_fallback(mut self, fallback_path: &str) -> Self {
        self.config.fallback_path = Some(fallback_path.to_string());
        self
    }

    pub fn set_permission(mut self, permission: Permission) -> Self {
        self.config.permission = permission;
        self
    }

    pub fn collect(self) -> Config {
        self.config
    }
}

pub async fn expose(config: Config) -> bool {
    let Config {
        addr,
        fallback_path,
        permission,
        server,
    } = config;

    server.start().await;
    server.set_permission(permission).await;

    let mut app = Router::new()
        .route("/query", post(http::run_query))
        .route("/rpc", get(websocket::handle_connection))
        .with_state(server);

    if let Some(fallback_path) = fallback_path {
        app = app.fallback(|| async move { Redirect::permanent(&fallback_path) });
    }

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .is_ok()
}
