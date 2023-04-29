mod filesystem;
pub use filesystem::FileStorage;

mod memory;
pub use memory::MemoryStorage;

use async_trait::async_trait;

/// Simple storage utility.
///
/// This is storage, core of the eight server.
/// Storage uses files or memory to store key-value data.
/// You shouldn't use storage itself, use with [`Server`] instead.
///
/// [`Server`]: ./struct.Server.html
#[async_trait]
pub trait Storage: Send + Sync + 'static {
    /// Create or replace a key in storage.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use eight::embedded::{Storage, FileStorage};
    /// # let storage = FileStorage::from_path("./set_storage_test");
    /// storage.set("bob".to_string(), "some session id".to_string()).await.unwrap();
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    async fn set(&self, key: String, value: String) -> super::Result<()>;

    /// Get a key from storage.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use eight::embedded::{Storage, FileStorage};
    /// # let storage = FileStorage::from_path("./get_storage_test");
    /// storage.set("bob".to_string(), "some session id".to_string()).await;
    ///
    /// let value = storage.get("bob".to_string()).await.unwrap();
    /// assert_eq!(value, "some session id".to_string());
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    async fn get(&self, key: String) -> super::Result<String>;

    /// Delete a key from storage.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use eight::embedded::{Storage, FileStorage};
    /// # let storage = FileStorage::from_path("./delete_storage_test");
    /// storage.set("bob".to_string(), "some session id".to_string()).await;
    ///
    /// // ...
    ///
    /// storage.delete("bob".to_string()).await.unwrap();
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    async fn delete(&self, key: String) -> super::Result<()>;

    /// Checks if key exists.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use eight::embedded::{Storage, FileStorage};
    /// # let storage = FileStorage::from_path("./exists_storage_test");
    /// storage.set("some".to_string(), "test".to_string()).await;
    /// assert_eq!(Ok(true), storage.exists("some".to_string()).await);
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    async fn exists(&self, key: String) -> super::Result<bool>;

    /// Find value and increment by given value.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use eight::embedded::{Storage, FileStorage};
    /// # let storage = FileStorage::from_path("./increment_storage_test");
    /// storage.set("bob_point".to_string(), "10".to_string()).await;
    ///
    /// let value = storage.increment("bob_point".to_string(), 5).await.unwrap();
    /// assert_eq!(value, 15);
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    async fn increment(&self, key: String, num: usize) -> super::Result<usize>;

    /// Find value and decrement by given value.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use eight::embedded::{Storage, FileStorage};
    /// # let storage = FileStorage::from_path("./decrement_storage_test");
    /// storage.set("bob_point".to_string(), "35".to_string()).await;
    ///
    /// let value = storage.decrement("bob_point".to_string(), 5).await.unwrap();
    /// assert_eq!(value, 30);
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    async fn decrement(&self, key: String, num: usize) -> super::Result<usize>;

    /// Search key from storage.
    ///
    /// # Note
    /// Search may give different results depend on implementation.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use eight::embedded::{Storage, FileStorage};
    /// # let storage = FileStorage::from_path("./search_storage_test");
    ///
    /// for i in 1..100 {
    ///   storage.set(format!("result{}", i), "test".to_string()).await.unwrap();
    /// }
    ///
    /// let results = storage.search("res".to_string()).await.unwrap();
    /// assert_eq!(results.len(), 99);
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    async fn search(&self, key: String) -> super::Result<Vec<String>>;

    /// Removes everything from storage.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use eight::embedded::{Storage, FileStorage};
    /// # let storage = FileStorage::from_path("./flush_storage_test");
    ///
    /// for i in 1..1000 {
    ///   storage.set(format!("result{}", i), "test".to_string()).await.unwrap();
    /// }
    ///
    /// storage.flush().await.unwrap();
    /// # });
    /// ```
    async fn flush(&self) -> super::Result<()>;
}
