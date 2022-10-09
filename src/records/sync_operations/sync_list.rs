use std::collections::HashMap;
use super::super::Recordable;
use crate::client::SyncClient;
use std::error::Error;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

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

pub fn records<T: Recordable + DeserializeOwned>(collection: &str, client: &SyncClient) -> Result<ListResponse<T>, Box<dyn Error>> {
    let list_response = client.get(
        format!("collections/{}/records", collection),
        None
    );

    match list_response {
        Ok(response) => {
            match response.json::<ListResponse<T>>() {
                Ok(parsed) => Ok(parsed),
                Err(err) => Err(Box::new(err) as Box<dyn Error>)
            }
        },
        Err(err) => Err(err)
    }
}
