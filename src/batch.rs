use crate::client::{Auth, Client};
use crate::httpc::Httpc;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct BatchOperation {
    method: String,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchResult {
    pub status: i32,
    pub body: serde_json::Value,
}

pub struct BatchRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    requests: Vec<BatchOperation>,
}

impl<'a> BatchRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client<Auth>) -> Self {
        Self { client, requests: Vec::new() }
    }

    pub fn create<T: Serialize>(mut self, collection: &str, record: &T) -> Result<Self> {
        self.requests.push(BatchOperation {
            method: "POST".to_string(),
            url: format!("/api/collections/{}/records", collection),
            body: Some(serde_json::to_value(record)?),
        });
        Ok(self)
    }

    pub fn update<T: Serialize>(mut self, collection: &str, id: &str, record: &T) -> Result<Self> {
        self.requests.push(BatchOperation {
            method: "PATCH".to_string(),
            url: format!("/api/collections/{}/records/{}", collection, id),
            body: Some(serde_json::to_value(record)?),
        });
        Ok(self)
    }

    pub fn upsert<T: Serialize>(mut self, collection: &str, record: &T) -> Result<Self> {
        self.requests.push(BatchOperation {
            method: "PUT".to_string(),
            url: format!("/api/collections/{}/records", collection),
            body: Some(serde_json::to_value(record)?),
        });
        Ok(self)
    }

    pub fn delete(mut self, collection: &str, id: &str) -> Self {
        self.requests.push(BatchOperation {
            method: "DELETE".to_string(),
            url: format!("/api/collections/{}/records/{}", collection, id),
            body: None,
        });
        self
    }

    pub fn call(&self) -> Result<Vec<BatchResult>> {
        #[derive(Serialize)]
        struct Payload<'a> {
            requests: &'a [BatchOperation],
        }
        let url = format!("{}/api/batch", self.client.base_url);
        let payload = serde_json::to_string(&Payload { requests: &self.requests })?;
        match Httpc::post(self.client, &url, payload) {
            Ok(response) => Ok(response.into_json::<Vec<BatchResult>>()?),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
}
