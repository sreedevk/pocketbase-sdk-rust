use super::super::Recordable;
use crate::client::Client;
use std::error::Error;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    code: u8,
    message: String,
    data: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedCollection<T> {
    page: u32,
    per_page: u32,
    total_items: u32,
    total_pages: u32,
    items: Vec<T> 
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ListResponse<T> {
    ErrorResponse(ErrorResponse),
    SuccessResponse(PaginatedCollection<T>)
}


pub async fn records<T: Recordable + DeserializeOwned>(collection: &str, client: &Client) -> Result<ListResponse<T>, Box<dyn Error>> {
    let list_response = client.get(
        format!("collections/{}/records", collection)
    ).await;

    match list_response {
        Ok(response) => {
            match response.json::<ListResponse<T>>().await {
                Ok(parsed) => Ok(parsed),
                Err(err) => Err(Box::new(err) as Box<dyn Error>)
            }
        },
        Err(err) => Err(err)
    }
}
