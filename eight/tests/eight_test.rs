use anyhow::Result;
use eight::{Request, Response, Server, Storage};
use std::str::FromStr;

#[tokio::test]
async fn simple_server() -> Result<()> {
    let storage = Storage::from_str("./server_test")?;
    let server = Server::new(storage);

    server.start().await;

    server
        .call(Request::Set("naber".into(), "iyi".into()))
        .await?
        .result()?;

    if let Response::Value(value) = server.call(Request::Get("naber".into())).await? {
        dbg!(value);
    } else {
        panic!();
    }

    server.call(Request::Flush).await?;
    Ok(())
}

#[tokio::test]
async fn simple_storage() -> Result<()> {
    let storage = Storage::from_str("./storage_test")?;

    storage
        .set("test".to_string(), "test value".to_string())
        .await?;

    let value = storage.get("test".to_string()).await?;
    assert_eq!(value, "test value".to_string());

    storage.flush().await?;

    Ok(())
}
