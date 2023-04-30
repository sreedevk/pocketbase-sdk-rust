use crate::client::{Auth, Client};
use crate::httpc::Httpc;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub system: bool,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub unique: bool,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldDeclaration<'a> {
    pub name: &'a str,
    pub r#type: &'a str,
    pub required: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionList {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i32,
    pub items: Vec<Collection>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub created: String,
    pub r#type: String,
    pub updated: String,
    pub name: String,
    pub schema: Vec<Field>,
}

#[derive(Clone, Debug)]
pub struct CollectionsManager<'a> {
    pub client: &'a Client<Auth>,
}

/*TODO: Add Auth Options & View Options for View & Auth Types*/
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDetails<'a> {
    pub id: Option<&'a str>,
    pub name: Option<&'a str>,
    pub r#type: Option<&'a str>,
    pub schema: Vec<FieldDeclaration<'a>>,
    pub system: bool,
    pub list_rule: Option<String>,
    pub view_rule: Option<String>,
    pub create_rule: Option<String>,
    pub update_rule: Option<String>,
    pub delete_rule: Option<String>,
    pub indexes: Vec<String>
}

#[derive(Debug, Clone)]
pub struct CollectionCreateRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub collection_details: Option<CollectionDetails<'a>>
}

#[derive(Clone, Debug)]
pub struct CollectionViewRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub name: &'a str,
}

#[derive(Clone, Debug)]
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
        let mut build_opts: Vec<(&str, &str)> = Vec::new();

        if let Some(filter_opts) = &self.filter { build_opts.push(("filter", filter_opts)) }
        if let Some(sort_opts) = &self.sort { build_opts.push(("sort", sort_opts)) }
        let per_page_opts = self.per_page.to_string();
        let page_opts = self.page.to_string();
        build_opts.push(("per_page", per_page_opts.as_str()));
        build_opts.push(("page", page_opts.as_str()));

        match Httpc::get(self.client, &url, Some(build_opts)) {
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
    pub fn view(&self, name: &'a str) -> CollectionViewRequestBuilder {
        CollectionViewRequestBuilder {
            client: self.client,
            name,
        }
    }

    pub fn create(&self, name: &'a str) -> CollectionCreateRequestBuilder {
       CollectionCreateRequestBuilder { client: self.client, collection_details: None } 
    }

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

impl<'a> CollectionViewRequestBuilder<'a> {
    pub fn call(&self) -> Result<Collection> {
        let url = format!("{}/api/collections/{}", self.client.base_url, self.name);
        match Httpc::get(self.client, &url, None) {
            Ok(result) => {
                let response = result.into_json::<Collection>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }
}
