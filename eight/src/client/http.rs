//! Client implementation for HTTP connections.

use super::messaging;
use crate::err;

/// Stateless HTTP client struct.
#[derive(Default, Debug, Clone)]
pub struct Client {
    host: String,
}

impl Client {
    /// Create new HTTP client from host.
    ///
    /// Since HTTP queries are executed from `/query` path, it appends "/query" to end of the string before creating struct.
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

    /// Execute a query and wait for [`messaging::Response`].
    ///
    /// ```no_run
    /// # async fn hi() {
    /// use eight::client::{messaging::QueryBuilder, http::Client};
    ///
    /// let client = Client::new("http://localhost:3000/");
    ///
    /// let request = QueryBuilder::new()
    ///   .add_query("delete $user;")
    ///   .bind("user", "bob")
    ///   .set_random_id()
    ///   .collect();
    ///
    /// let response = client.execute(request).await;
    /// # }
    /// ```
    pub async fn execute(&self, request: messaging::Request) -> super::Result<messaging::Response> {
        let client = reqwest::Client::default();

        let response = client
            .post(&self.host)
            .json(&request)
            .send()
            .await
            .map_err(|_| err!(client, HTTPRequestFail))?;

        let body = response
            .json::<messaging::Response>()
            .await
            .map_err(|_| err!(client, ReadBodyFail))?;

        Ok(body)
    }
}
