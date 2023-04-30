use crate::client::{Auth, Client};
use anyhow::Result;
use crate::httpc::Httpc;
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct RecordsManager<'a> {
    pub client: &'a Client<Auth>,
    pub name: &'a str,
}

#[derive(Debug, Clone)]
pub struct RecordsListRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub collection_name: &'a str,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordList<T> {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i32,
    pub items: Vec<T>,
}

impl<'a> RecordsListRequestBuilder<'a> {
    pub fn call<T: Default + DeserializeOwned>(&self) -> Result<RecordList<T>> {
        let url = format!("{}/api/collections/{}/records", self.client.base_url, self.collection_name);
        match Httpc::get(self.client, &url, None) {
            Ok(result) => {
                let response = result.into_json::<RecordList<T>>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }

    pub fn filter(&self, filter_opts: &str) -> Self {
        Self {
            filter: Some(filter_opts.to_string()),
            ..self.clone()
        }
    }

    pub fn sort(&self, sort_opts: &str) -> Self {
        Self {
            sort: Some(sort_opts.to_string()),
            ..self.clone()
        }
    }

    pub fn page(&self, page: i32) -> Self {
        Self {
            page,
            ..self.clone()
        }
    }

    pub fn per_page(&self, per_page: i32) -> Self {
        Self {
            per_page,
            ..self.clone()
        }
    }
}

pub struct RecordViewRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub collection_name: &'a str,
    pub identifier: &'a str,
}

impl<'a> RecordViewRequestBuilder<'a> {
    pub fn call<T: Default + DeserializeOwned>(&self) -> Result<T> {
        let url = format!("{}/api/collections/{}/records/{}", self.client.base_url, self.collection_name, self.identifier);
        match Httpc::get(self.client, &url, None) {
            Ok(result) => {
                let response = result.into_json::<T>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }
}

impl<'a> RecordsManager<'a> {
    pub fn view(&self, identifier: &'a str) -> RecordViewRequestBuilder<'a> {
        RecordViewRequestBuilder {
            identifier,
            client: self.client,
            collection_name: self.name,
        }
    }

    pub fn list(&self) -> RecordsListRequestBuilder<'a> {
        RecordsListRequestBuilder {
            client: self.client,
            collection_name: self.name,
            filter: None,
            sort: None,
            page: 1,
            per_page: 100,
        }
    }
}
