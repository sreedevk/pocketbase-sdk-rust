use crate::client::SyncClient;
use serde::{Serialize, Deserialize};
use std::error::Error;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FailureResponse {
    code: String,
    message: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CreateResponse<T> {
    SuccessResponse(T),
    FailureResponse(FailureResponse)
}

pub fn record<T: Serialize + DeserializeOwned>(collection: &str, changeset: &T, client: &SyncClient) -> Result<CreateResponse<T>, Box<dyn Error>> {
    let url = format!("collections/{}/records", collection);
    match client.post::<T>(url, &changeset) {
        Ok(response) => {
           match response.json::<T>() {
                Ok(parsed) => Ok(CreateResponse::SuccessResponse(parsed)),
                Err(e) => Err(Box::new(e) as Box<dyn Error>)
            }
        },
        Err(e) => Err(e)
    }
}
