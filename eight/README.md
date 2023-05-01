## Installation

To add this crate as a dependency, simply run

```bash
cargo add eight
```

Eight library supports both embedded and client usage. You can enable `client` feature to use official client implementation for `eight-serve`.

## Embedded Usage

Eight currently ships two default storage implementation. An example for filesystem storage:

```rust no_run
use eight::{embedded::{self, messaging::{Request, Response}, server::Server, storage::filesystem}};
use std::collections::HashMap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> embedded::Result<()> {
    // create filesystem storage
    let storage = filesystem::Storage::from_path("/path/to/store");

    // create new server from storage
    let server = Server::new(storage);

    // start listener in another task
    server.start().await;

    // send a request to server and wait for response
    let response = server.call(Request::Set("pipi".to_string(), "hello world".to_string())).await?;
    assert_eq!(response, Response::Ok);

    // query language usage
    let mut env = HashMap::<String, String>::new();
    env.insert("user".to_string(), "pipi".to_string());

    let results = server.query("
        get $user;
        delete $user;
    ", env).await?;

    assert_eq!(results.len(), 2);
    assert_eq!(results[0], Response::Text("hello world".to_string()));
    assert_eq!(results[1], Response::Ok);

    // clear database before existing
    server.call(Request::Flush).await?;

    Ok(())
}
```

## Client Usage

### HTTP Client

```rust no_run
use eight::client::{self, http, messaging};

#[tokio::main(flavor = "current_thread")]
async fn main() -> client::Result<()> {
    // crate new stateless client
    let client = http::Client::new("http://127.0.0.1:8080");

    // prepare request
    let request = messaging::QueryBuilder::new()
        .add_query("set $key $value;")
        .add_query("get $key;")
        .bind("key", "bob")
        .bind("value", "some value")
        .set_random_id()
        .collect();

    // send request
    let response = client.execute(request).await?;
    assert_eq!(response.results.len(), 2);

    Ok(())
}

```

### WebSocket Client

```rust no_run
use eight::client::{self, websocket, messaging};

#[tokio::main(flavor = "current_thread")]
async fn main() -> client::Result<()> {
    // connect to websocket
    let client = websocket::Client::connect("ws://127.0.0.1:8080").await?;

    // start message broker
    client.start().await;

    // prepare request
    let request = messaging::QueryBuilder::new()
        .add_query("set $key $value;")
        .add_query("get $key;")
        .bind("key", "bob")
        .bind("value", "some value")
        .set_random_id()
        .collect();

    // send request and wait for response
    let response = client.call(request).await?;
    assert_eq!(response.results.len(), 2);

    Ok(())
}
```

## Documentation

You can find documentation on [docs.rs](https://docs.rs/eight).
