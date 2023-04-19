## Installation

To add this crate as a dependency, simply run

```bash
cargo add eight
```

## Simple Usage

Eight library keeps itself simple as possible. When you look at source code you may see a lot of things but we only expose what you need.

```rust
use eight::{EightResult, Server, Request, Response};
use std::{collections::HashMap, str::FromStr};

#[tokio::main]
async fn main() -> EightResult<()> {
    let server = Server::from_str("/path/to/store").unwrap();

    // start listener in another task
    server.start().await;

    // send a request to server and wait for response
    let response = server.call(Request::Set("icecat".into(), "hello world".into())).await?;
    assert_eq!(response, Response::Ok);

    // query language usage
    let mut env = HashMap::<String, String>::new();
    env.insert("user".into(), "icecat".into());

    let results = server.query("
        get $user;
        delete $user;
    ", env).await?;

    assert_eq!(results.len(), 2);
    assert_eq!(results[0], Response::Value("hello world".into()));
    assert_eq!(results[1], Response::Ok);

    // clear database before existing
    server.call(Request::Flush).await?;

    Ok(())
}
```

## Documentation

You can find documentation on [docs.rs](https://docs.rs/eight).
