use crate::{EightResult, Storage};
use std::str::FromStr;

#[tokio::test]
async fn test_storage() -> EightResult<()> {
    let storage = Storage::from_str("./storage_test").unwrap();

    storage
        .set("test".to_string(), "test value".to_string())
        .await?;

    let value = storage.get("test".to_string()).await?;
    assert_eq!(value, "test value".to_string());

    storage.flush().await?;

    Ok(())
}
