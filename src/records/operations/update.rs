use crate::client::Client;
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

pub async fn record<T: Serialize + DeserializeOwned>(collection: &str, id: &str, changeset: &T, client: &Client) -> Result<CreateResponse<T>, Box<dyn Error>> {
    let url = format!("collections/{}/records/{}", collection, id);
    match client.patch::<T>(url, &changeset).await {
        Ok(response) => {
           match response.json::<T>().await {
                Ok(parsed) => Ok(CreateResponse::SuccessResponse(parsed)),
                Err(e) => Err(Box::new(e) as Box<dyn Error>)
            }
        },
        Err(e) => Err(e)
    }
}
