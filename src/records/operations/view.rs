use super::PocketbaseOperationError;
use crate::client::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    code: u8,
    message: String,
    data: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ViewResponse<T> {
    ErrorResponse(ErrorResponse),
    SuccessResponse(T),
}

pub async fn record<T: DeserializeOwned>(
    collection: &str,
    id: &str,
    client: &Client,
) -> Result<ViewResponse<T>, PocketbaseOperationError> {
    let http_request = client
        .get::<HashMap<String, String>>(&format!("collections/{}/records/{}", collection, id), None)
        .await;

    match http_request {
        Ok(request) => {
            let http_client = surf::client();
            match http_client.recv_json(request).await {
                Ok(response) => Ok(response),
                Err(_) => Err(PocketbaseOperationError::Failed),
            }
        }
        Err(_) => Err(PocketbaseOperationError::Failed),
    }
}
