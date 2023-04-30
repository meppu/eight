use eight::{
    embedded::{self, server::Server, storage::memory},
    expose,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> embedded::Result<()> {
    let storage = memory::Storage::new();
    let server = Server::new(storage);

    let expose_config = expose::ConfigBuilder::from_server(server)
        .bind(SocketAddr::from(([127, 0, 0, 1], 42069)))
        .collect();

    expose::expose(expose_config).await;
    Ok(())
}
