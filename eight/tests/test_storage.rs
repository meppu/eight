use anyhow::Result;
use eight::Storage;
use std::str::FromStr;

#[tokio::test]
async fn simple_storage() -> Result<()> {
    let storage = Storage::from_str("./lol")?;

    storage
        .set("test".to_string(), "test value".to_string())
        .await?;

    let value = storage.get("test".to_string()).await?;
    assert_eq!(value, "test value".to_string());

    storage.flush().await?;

    Ok(())
}
