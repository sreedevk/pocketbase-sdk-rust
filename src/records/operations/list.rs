use std::collections::HashMap;
use crate::client::Client;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use super::PocketbaseOperationError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    code: u8,
    message: String,
    data: HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedRecordList<T> {
    pub page: u32,
    pub per_page: u32,
    pub total_items: u32,
    pub total_pages: u32,
    pub items: Vec<T> 
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ListResponse<T> {
    ErrorResponse(ErrorResponse),
    SuccessResponse(PaginatedRecordList<T>)
}

pub async fn records<T: DeserializeOwned>(collection: &str, client: &Client, params: Option<&HashMap<String, String>>) -> Result<ListResponse<T>, PocketbaseOperationError> {
    let http_request = client.get(
        format!("collections/{}/records", collection),
        params
    ).await;
    
    match http_request {
        Ok(request) => {
            let http_client = surf::client();
            match http_client.recv_json(request).await {
                Ok(response) => Ok(response),
                Err(_) => Err(PocketbaseOperationError::Failed)
            }
        },
        Err(_) => Err(PocketbaseOperationError::Failed)
    }
}
