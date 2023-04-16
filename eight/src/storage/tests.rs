use crate::Storage;
use std::str::FromStr;

#[tokio::test]
async fn test_storage() -> anyhow::Result<()> {
    let storage = Storage::from_str("./storage_test")?;

    storage
        .set("test".to_string(), "test value".to_string())
        .await?;

    let value = storage.get("test".to_string()).await?;
    assert_eq!(value, "test value".to_string());

    storage.flush().await?;

    Ok(())
}
