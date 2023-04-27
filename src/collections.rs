use crate::client::{Auth, Client};
use crate::httpc::Httpc;
use crate::types::FieldType;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub system: bool,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub unique: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionList {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i32,
    pub items: Vec<Collection>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub created: String,
    pub updated: String,
    pub name: String,
    pub schema: Vec<Field>,
}

#[derive(Clone)]
pub struct CollectionsManager<'a> {
    pub client: &'a Client<Auth>,
}

#[derive(Clone)]
pub struct CollectionListRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub per_page: i32,
    pub page: i32,
}

impl<'a> CollectionListRequestBuilder<'a> {
    pub fn call(&self) -> Result<CollectionList> {
        let url = format!("{}/api/collections", self.client.base_url);
        let mut build_opts = HashMap::new();
        if let Some(filter_opts) = &self.filter {
            build_opts.insert("filter".to_string(), filter_opts.clone());
        }

        if let Some(sort_opts) = &self.sort {
            build_opts.insert("sort".to_string(), sort_opts.clone());
        }

        build_opts.insert("per_page".to_string(), self.per_page.to_string());
        build_opts.insert("page".to_string(), self.page.to_string());

        match Httpc::get(self.client, &url) {
            Ok(result) => {
                let response = result.into_json::<CollectionList>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }

    pub fn filter(&self, filter_opts: String) -> Self {
        Self {
            filter: Some(filter_opts),
            ..self.clone()
        }
    }

    pub fn per_page(&self, per_page_count: i32) -> Self {
        Self {
            per_page: per_page_count,
            ..self.clone()
        }
    }

    pub fn page(&self, page_count: i32) -> Self {
        Self {
            page: page_count,
            ..self.clone()
        }
    }

    pub fn sort(&self, sort_opts: String) -> Self {
        Self {
            sort: Some(sort_opts),
            ..self.clone()
        }
    }
}

impl<'a> CollectionsManager<'a> {
    pub fn list(&self) -> CollectionListRequestBuilder {
        CollectionListRequestBuilder {
            client: self.client,
            filter: None,
            sort: None,
            per_page: 100,
            page: 1,
        }
    }
}
