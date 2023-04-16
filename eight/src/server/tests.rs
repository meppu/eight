use crate::{Request, Response, Server, Storage};
use std::str::FromStr;

#[tokio::test]

async fn test_server() -> anyhow::Result<()> {
    let storage = Storage::from_str("./server_test")?;
    let server = Server::new(storage);

    server.start().await;

    server
        .call(Request::Set("test".into(), "iyi".into()))
        .await?
        .result()?;

    if let Response::Value(value) = server.call(Request::Get("test".into())).await? {
        assert_eq!(value, "iyi".to_string());
    } else {
        panic!();
    }

    server.call(Request::Flush).await?;
    Ok(())
}

#[tokio::test]
async fn test_increment_decrement() -> anyhow::Result<()> {
    let storage = Storage::from_str("./inc_dec_test")?;
    let server = Server::new(storage);

    server.start().await;

    server
        .call(Request::Set("test".into(), "10".into()))
        .await?
        .result()?;

    if let Response::Value(value) = server.call(Request::Get("test".into())).await? {
        assert_eq!(value, "10".to_string());
    } else {
        panic!();
    }

    server.call(Request::Increment("test".into(), 10)).await?;
    server.call(Request::Decrement("test".into(), 5)).await?;

    if let Response::Value(value) = server.call(Request::Get("test".into())).await? {
        assert_eq!(value, "15".to_string());
    } else {
        panic!();
    }

    server.call(Request::Flush).await?;
    Ok(())
}
