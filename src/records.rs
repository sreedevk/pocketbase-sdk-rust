use crate::client::{Auth, Client};
use crate::httpc::Httpc;
use anyhow::{anyhow, Result};
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};

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
        let url = format!(
            "{}/api/collections/{}/records",
            self.client.base_url, self.collection_name
        );

        let mut build_opts: Vec<(&str, &str)> = vec![];
        if let Some(filter_opts) = &self.filter {
            build_opts.push(("filter", filter_opts))
        }
        if let Some(sort_opts) = &self.sort {
            build_opts.push(("sort", sort_opts))
        }
        let per_page_opts = self.per_page.to_string();
        let page_opts = self.page.to_string();
        build_opts.push(("perPage", per_page_opts.as_str()));
        build_opts.push(("page", page_opts.as_str()));

        match Httpc::get(self.client, &url, Some(build_opts)) {
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
        let url = format!(
            "{}/api/collections/{}/records/{}",
            self.client.base_url, self.collection_name, self.identifier
        );
        match Httpc::get(self.client, &url, None) {
            Ok(result) => {
                let response = result.into_json::<T>()?;
                Ok(response)
            }
            Err(e) => Err(anyhow!("error: {}", e)),
        }
    }
}

impl<'a> RecordDestroyRequestBuilder<'a> {
    pub fn call(&self) -> Result<()> {
        let url = format!(
            "{}/api/collections/{}/records/{}",
            self.client.base_url, self.collection_name, self.identifier
        );
        match Httpc::delete(self.client, url.as_str()) {
            Ok(result) => {
                if result.status() == 204 {
                    Ok(())
                } else {
                    Err(anyhow!("Failed to delete"))
                }
            }
            Err(e) => Err(anyhow!("error: {}", e)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RecordDestroyRequestBuilder<'a> {
    pub identifier: &'a str,
    pub client: &'a Client<Auth>,
    pub collection_name: &'a str,
}

#[derive(Debug, Clone)]
pub struct RecordDeleteAllRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub collection_name: &'a str,
    pub filter: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct RecordCreateRequestBuilder<'a, T: Serialize + Clone> {
    pub client: &'a Client<Auth>,
    pub collection_name: &'a str,
    pub record: T,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateResponse {
    #[serde(rename = "@collectionName")]
    pub collection_name: Option<String>,
    #[serde(rename = "@collectionId")]
    pub collection_id: Option<String>,
    pub id: String,
    pub updated: String,
    pub created: String,
}

impl<'a, T: Serialize + Clone> RecordCreateRequestBuilder<'a, T> {
    pub fn call(&self) -> Result<CreateResponse> {
        let url = format!(
            "{}/api/collections/{}/records",
            self.client.base_url, self.collection_name
        );
        let payload = serde_json::to_string(&self.record).map_err(anyhow::Error::from)?;
        match Httpc::post(self.client, &url, payload) {
            Ok(result) => {
                let response = result.into_json::<CreateResponse>()?;
                Ok(response)
            }
            Err(e) => Err(anyhow!("error: {}", e)),
        }
    }
}

pub struct RecordUpdateRequestBuilder<'a, T: Serialize + Clone> {
    pub record: T,
    pub collection_name: &'a str,
    pub client: &'a Client<Auth>,
    pub id: &'a str,
}

impl<'a, T: Serialize + Clone> RecordUpdateRequestBuilder<'a, T> {
    pub fn call(&self) -> Result<T> {
        let url = format!(
            "{}/api/collections/{}/records/{}",
            self.client.base_url, self.collection_name, self.id
        );
        let payload = serde_json::to_string(&self.record).map_err(anyhow::Error::from)?;
        match Httpc::patch(self.client, &url, payload) {
            Ok(result) => {
                result.into_json::<CreateResponse>()?;
                Ok(self.record.clone())
            }
            Err(e) => Err(anyhow!("error: {}", e)),
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

    pub fn destroy(&self, identifier: &'a str) -> RecordDestroyRequestBuilder<'a> {
        RecordDestroyRequestBuilder {
            identifier,
            client: self.client,
            collection_name: self.name,
        }
    }

    pub fn update<T: Serialize + Clone>(
        &self,
        identifier: &'a str,
        record: T,
    ) -> RecordUpdateRequestBuilder<'a, T> {
        RecordUpdateRequestBuilder {
            client: self.client,
            collection_name: self.name,
            id: identifier,
            record,
        }
    }

    pub fn create<T: Serialize + Clone>(&self, record: T) -> RecordCreateRequestBuilder<'a, T> {
        RecordCreateRequestBuilder {
            record,
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
