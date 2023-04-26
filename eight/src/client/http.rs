use super::messaging;

pub struct Client {
    host: String,
}

impl Client {
    pub fn new(host: String) -> Self {
        let mut host = host;
        host.push_str("/query");

        Self { host }
    }

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
