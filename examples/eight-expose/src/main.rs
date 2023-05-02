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

    // The following `thread` variable and if-else statement is for testing
    // the example on CI and it can be removed safely from the code.
    //
    // ```
    // expose::expose(expose_config).await;
    // ```
    let thread = tokio::spawn(expose::expose(expose_config));
    if std::env::var("CI").is_ok() {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    } else {
        thread.await.unwrap();
    }

    Ok(())
}
