use crate::client::{Client, Auth};
use crate::httpc::Httpc;
use crate::types::FieldType;
use anyhow::Result;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Field {
    pub system: bool,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub unique: bool,
}

#[derive(Debug, Deserialize)]
struct ListResponse {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i32,
    pub items: Vec<Collection>,
}

#[derive(Debug, Deserialize)]
pub struct Collection {
    pub id: String,
    pub created: String,
    pub updated: String,
    pub name: String,
    pub schema: Vec<Field>,
}

pub struct CollectionsManager<'a> {
    pub client: &'a Client<Auth>
}

impl<'a> CollectionsManager<'a> {
    pub fn list(&self) -> Result<Vec<Collection>> {
        let url = format!("{}/api/collections", self.client.base_url);
        match Httpc::post(self.client, &url, HashMap::new()) {
            Ok(result) => {
                let response = result.into_json::<ListResponse>();
                Ok(vec![])
            },
            Err(e) => Ok(vec![]),
        }
    }
}
