## Installation

To add this crate as a dependency, simply run

```bash
cargo add eight
```

Eight library supports both embedded and client usage. By default it comes with `embedded` feature enabled. You can also enable `client` feature to use official client implementation for `eight-serve`.

## Embedded Usage

```rust ignore
use eight::{embedded::Server, messaging::{Request, Response}};
use std::{collections::HashMap, str::FromStr};

#[tokio::main]
async fn test() -> eight::Result<()> {
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
    assert_eq!(results[0], Response::Text("hello world".into()));
    assert_eq!(results[1], Response::Ok);

    // clear database before existing
    server.call(Request::Flush).await?;

    Ok(())
}
```

## Client Usage

TODO

## Documentation

You can find documentation on [docs.rs](https://docs.rs/eight).
