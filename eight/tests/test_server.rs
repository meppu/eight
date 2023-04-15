use anyhow::Result;
use eight::{Request, Response, Server, Storage};
use std::str::FromStr;

#[tokio::test]
async fn simple_storage() -> Result<()> {
    let storage = Storage::from_str("./lol")?;
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
