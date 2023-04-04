use super::PocketbaseOperationError;
use crate::client::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SuccessResponse {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailureResponse {
    code: String,
    message: String,
    data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
enum DeleteResponse {
    SuccessResponse(SuccessResponse),
    FailureResponse(FailureResponse),
}

pub async fn record(
    collection: &str,
    id: &str,
    client: &Client,
) -> Result<(), PocketbaseOperationError> {
    let url = format!("/api/collections/{}/records/{}", collection, id);
    match client.delete(&url).await {
        Ok(request) => {
            let http_client = surf::client();
            match http_client.recv_string(request).await {
                Ok(_) => Ok(()),
                Err(_) => Err(PocketbaseOperationError::Failed),
            }
        }
        Err(_) => Err(PocketbaseOperationError::Failed),
    }
}
