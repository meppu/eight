use crate::{filesystem, EightError, EightResult};
use std::{path::PathBuf, str::FromStr};

/// Simple storage utility.
///
/// This is storage, core of the eight server.
/// Storage uses files to store key-value data.
/// You shouldn't use storage itself, use [`Server`] instead.
///
/// Cloning storage is cheap since it only stores path buffer inside of it.
///
/// [`Server`]: ./struct.Server.html
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Storage {
    path: PathBuf,
}

impl FromStr for Storage {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            path: PathBuf::from_str(s)?,
        })
    }
}

impl Storage {
    /// Create new storage.
    ///
    /// This function returns same result with default value:
    ///
    /// ```
    /// use eight::Storage;
    ///
    /// let default = Storage::default();
    /// let new = Storage::new();
    ///
    /// assert_eq!(default, new);
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Create or replace a key in storage.
    ///
    /// This function stores key in given directory and returns nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::Storage;
    /// use std::str::FromStr;
    ///
    /// let storage = Storage::from_str("./set_storage_test").unwrap();
    ///
    /// if let Err(error) = storage.set("icecat".into(), "some session id".into()).await {
    ///   panic!("{}", error.to_string());
    /// }
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    pub async fn set(&self, key: String, value: String) -> EightResult<()> {
        let mut path = filesystem::create_path(&self.path, &key)?;
        filesystem::write(&mut path, value).await
    }

    /// Get a key from storage.
    ///
    /// This function reads file and returns it is content as string.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::Storage;
    /// use std::str::FromStr;
    ///
    /// let storage = Storage::from_str("./get_storage_test").unwrap();
    ///
    /// storage.set("icecat".into(), "some session id".into()).await;
    ///
    /// match storage.get("icecat".into()).await {
    ///   Ok(value) => assert_eq!(value, "some session id".to_string()),
    ///   Err(error) => panic!("{}", error.to_string()),
    /// }
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    pub async fn get(&self, key: String) -> EightResult<String> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::read(&path).await
    }

    /// Delete a key from storage.
    ///
    /// This function removes file and returns nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::Storage;
    /// use std::str::FromStr;
    ///
    /// let storage = Storage::from_str("./delete_storage_test").unwrap();
    ///
    /// storage.set("icecat".into(), "some session id".into()).await;
    ///
    /// if let Err(error) = storage.delete("icecat".into()).await {
    ///   panic!("{}", error.to_string());
    /// }
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    pub async fn delete(&self, key: String) -> EightResult<()> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::delete(&path).await
    }

    /// Find value and increment by given value.
    ///
    /// This function reads file, trying to parse it as unsigned integer, updates it and returns new value as unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::Storage;
    /// use std::str::FromStr;
    ///
    /// let storage = Storage::from_str("./increment_storage_test").unwrap();
    ///
    /// storage.set("icecat_point".into(), "10".into()).await;
    ///
    /// match storage.increment("icecat_point".into(), 5).await {
    ///   Ok(value) => assert_eq!(value, 15),
    ///   Err(error) => panic!("{}", error.to_string()),
    /// }
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    pub async fn increment(&self, key: String, add: usize) -> EightResult<usize> {
        let mut path = filesystem::create_path(&self.path, &key)?;

        let raw = filesystem::read(&path).await?;

        if let Ok(value) = raw.parse::<usize>() {
            let new = value + add;

            filesystem::write(&mut path, new.to_string()).await?;
            Ok(new)
        } else {
            Err(EightError::UIntParseFail)
        }
    }

    /// Find value and decrement by given value.
    ///
    /// This function reads file, trying to parse it as unsigned integer, updates it and returns new value as unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::Storage;
    /// use std::str::FromStr;
    ///
    /// let storage = Storage::from_str("./decrement_storage_test").unwrap();
    ///
    /// storage.set("icecat_point".into(), "10".into()).await;
    ///
    /// match storage.decrement("icecat_point".into(), 5).await {
    ///   Ok(value) => assert_eq!(value, 5),
    ///   Err(error) => panic!("{}", error.to_string()),
    /// }
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    pub async fn decrement(&self, key: String, add: usize) -> EightResult<usize> {
        let mut path = filesystem::create_path(&self.path, &key)?;

        let raw = filesystem::read(&path).await?;

        if let Ok(value) = raw.parse::<usize>() {
            let new = value - add;

            filesystem::write(&mut path, new.to_string()).await?;
            Ok(new)
        } else {
            Err(EightError::UIntParseFail)
        }
    }

    /// Checks if key exists
    ///
    /// This function returns a boolean on success.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::Storage;
    /// use std::str::FromStr;
    ///
    /// let storage = Storage::from_str("./decrement_storage_test").unwrap();
    ///
    /// storage.set("some".into(), "test".into()).await;
    ///
    /// match storage.exists("some".into()).await {
    ///   Ok(true) => storage.delete("some".into()).await.unwrap(),
    ///   Ok(false) => panic!("it doesn't exists"),
    ///   Err(error) => panic!("{}", error.to_string()),
    /// }
    ///
    /// # storage.flush().await;
    /// # });
    /// ```
    pub async fn exists(&self, key: String) -> EightResult<bool> {
        let path = filesystem::create_path(&self.path, &key)?;
        filesystem::exists(&path).await
    }

    /// Removes everything from storage.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use eight::Storage;
    /// use std::str::FromStr;
    ///
    /// let storage = Storage::from_str("./flush_storage_test").unwrap();
    ///
    /// for i in 1..100 {
    ///   storage.set(i.to_string(), "test".into()).await;
    /// }
    ///
    /// if let Err(error) = storage.flush().await {
    ///   panic!("{}", error.to_string());
    /// }
    /// # });
    /// ```
    pub async fn flush(&self) -> EightResult<()> {
        filesystem::flush(&self.path).await
    }
}
