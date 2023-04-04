use super::PocketbaseOperationError;
use crate::client::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FailureResponse {
    code: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UpdateResponse<T> {
    SuccessResponse(T),
    FailureResponse(FailureResponse),
}

pub async fn record<T: Serialize + DeserializeOwned>(
    collection: &str,
    id: &str,
    changeset: &T,
    client: &Client,
) -> Result<UpdateResponse<T>, PocketbaseOperationError> {
    let url = format!("collections/{}/records/{}", collection, id);
    match client.patch::<T>(&url, &changeset).await {
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
