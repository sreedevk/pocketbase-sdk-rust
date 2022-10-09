use crate::client::Client;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    code: u8,
    message: String,
    data: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ViewResponse<T> {
    ErrorResponse(ErrorResponse),
    SuccessResponse(T)
}

pub async fn record<T: DeserializeOwned>(collection: &str, id: &str, client: &Client) -> Result<ViewResponse<T>, Box<dyn Error>> {
    let response = client.get(
        format!("collections/{}/records/{}", collection, id),
    ).await;

    match response {
        Ok(resp) => { 
            match resp.json::<ViewResponse<T>>().await {
                Ok(parsed) => Ok(parsed),
                Err(e) => Err(Box::new(e) as Box<dyn Error>)
            }
        }
        Err(err) => Err(err)

    }
}
