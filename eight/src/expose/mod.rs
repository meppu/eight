//! Create web server for [`Server`]. This web server allows you to host embedded server over the network. Supports both HTTP and WebSocket connections to run queries.

mod http;
mod websocket;

use crate::embedded::{Permission, Server};
use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

/// Start server with given config. Returns true if success.
///
/// Note to mention, [`expose`] uses [axum](https://docs.rs/axum/) under the hood.
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

/// Config structure for [`expose`] function.
#[derive(Debug)]
pub struct Config {
    addr: SocketAddr,
    fallback_path: Option<String>,
    permission: Permission,
    server: Server,
}

/// Builder for [`Config`] struct.
///
/// ```no_run
/// # use eight::{embedded::Server, expose::ConfigBuilder};
/// # use std::{net::SocketAddr, str::FromStr};
/// # let server = Server::from_str("/test").unwrap();
/// # let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
/// let config = ConfigBuilder::from_server(server)
///   .set_fallback("https://surrealdb.com")
///   .bind(addr)
///   .collect();
///
/// ```
#[derive(Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Create new [`ConfigBuilder`] from [`Server`].
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

    /// Set bind adress.
    pub fn bind(mut self, addr: SocketAddr) -> Self {
        self.config.addr = addr;
        self
    }

    /// Set fallback url or path.
    pub fn set_fallback(mut self, fallback_path: &str) -> Self {
        self.config.fallback_path = Some(fallback_path.to_string());
        self
    }

    /// Set server permission.
    pub fn set_permission(mut self, permission: Permission) -> Self {
        self.config.permission = permission;
        self
    }

    /// Collect [`Config`] result.
    pub fn collect(self) -> Config {
        self.config
    }
}
