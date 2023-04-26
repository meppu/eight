//! Client implementation for HTTP connections.

use super::messaging;

/// Stateless client struct.
pub struct Client {
    host: String,
}

impl Client {
    /// Create new http client from host.
    ///
    /// Since http queries are executed from `/query` path, it appends "/query" to end of the string before creating struct.
    ///
    /// ```no_run
    /// use eight::client::http::Client;
    ///
    /// let client = Client::new("http://localhost:3000/");
    /// ```
    pub fn new(host: &str) -> Self {
        let mut host = host.to_string();
        host.push_str("/query");

        Self { host }
    }

    /// Execute a query.
    ///
    /// ```no_run
    /// # async fn hi() {
    /// use eight::client::{messaging::QueryBuilder, http::Client};
    ///
    /// let client = Client::new("http://localhost:3000/");
    ///
    /// let request = QueryBuilder::new()
    ///   .add_query("set $user 0")
    ///   .bind("user", "bob")
    ///   .set_id("some_unique_id_to_handle")
    ///   .collect();
    ///
    /// let response = client.execute(request).await;
    /// # }
    /// ```
    pub async fn execute(&self, request: messaging::Request) -> super::Result<messaging::Response> {
        let raw_request = serde_json::to_string(&request).unwrap_or_default();
        let client = reqwest::Client::new();

        let response = client
            .post(&self.host)
            .body(raw_request)
            .send()
            .await
            .map_err(|_| super::Error::HTTPRequestFail)?;

        let body = response
            .text()
            .await
            .map_err(|_| super::Error::ReadBodyFail)?;

        Ok(serde_json::from_str::<messaging::Response>(&body).unwrap())
    }
}
