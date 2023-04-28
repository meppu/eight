use std::net::SocketAddr;

use super::http;
use super::messaging;
use super::websocket;
use crate::embedded;
use crate::expose;

#[tokio::test]
async fn http_client() -> super::Result<()> {
    let server = embedded::Server::from_path("/tmp/http_client_test");

    let expose_config = expose::ConfigBuilder::from_server(server)
        .bind(SocketAddr::from(([127, 0, 0, 1], 42069)))
        .collect();

    tokio::spawn(expose::expose(expose_config));

    let client = http::Client::new("http://localhost:42069");

    let request = messaging::QueryBuilder::from_id("testing")
        .add_query("set $key $value;")
        .add_query("get $key;")
        .bind("key", "bob")
        .bind("value", "some value")
        .collect();

    let response = client.execute(request).await?;

    assert_eq!(response.id, "testing".to_string());
    assert_eq!(response.results.len(), 2);

    Ok(())
}

#[tokio::test]
async fn websocket_client() -> super::Result<()> {
    let server = embedded::Server::from_path("/tmp/websocket_client_test");

    let expose_config = expose::ConfigBuilder::from_server(server)
        .bind(SocketAddr::from(([127, 0, 0, 1], 42070)))
        .collect();

    tokio::spawn(expose::expose(expose_config));

    let client = websocket::Client::connect("ws://localhost:42070").await?;
    client.start().await;

    let request = messaging::QueryBuilder::from_id("testing")
        .add_query("set $key $value;")
        .add_query("get $key;")
        .bind("key", "bob")
        .bind("value", "some value")
        .collect();

    let response = client.call(request).await?;

    assert_eq!(response.id, "testing".to_string());
    assert_eq!(response.results.len(), 2);

    Ok(())
}
