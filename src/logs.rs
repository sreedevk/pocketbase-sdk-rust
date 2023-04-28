use crate::httpc::Httpc;
use crate::client::{Client, Auth};
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

pub struct LogManager<'a> {
    pub client: &'a Client<Auth>
}

#[derive(Debug, Clone)]
pub struct LogListRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub page: i32,
    pub per_page: i32,
    pub sort: Option<&'a str>,
    pub filter: Option<&'a str>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogListItem {
    id: String,
    created: String,
    updated: String,
    url: String,
    method: String,
    status: i32,
    ip: Option<String>,
    referer: String,
    user_agent: String,
    meta: HashMap<String, String>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogList {
    pub page: i32,
    pub per_page: i32,
    total_items: i32,
    items: Vec<LogListItem>
}

impl<'a> LogListRequestBuilder<'a> {
    pub fn page(&self, page_count: i32) -> Self {
        LogListRequestBuilder {
            page: page_count,
            ..self.clone()
        }
    }

    pub fn per_page(&self, per_page_count: i32) -> Self {
        LogListRequestBuilder {
            per_page: per_page_count,
            ..self.clone()
        }
    }

    pub fn filter(&self, filter_opts: &'a str) -> Self {
        LogListRequestBuilder { filter: Some(filter_opts), ..self.clone() }
    }

    pub fn sort(&self, sort_opts: &'a str) -> Self {
        LogListRequestBuilder { sort: Some(sort_opts), ..self.clone() }
    }

    pub fn call(&self) -> Result<LogList> {
        let url = format!("{}/api/logs/requests", self.client.base_url);
        let mut build_opts = HashMap::new();

        if let Some(sort_opts) = &self.sort {
            build_opts.insert("sort".to_string(), sort_opts.to_string());
        }

        if let Some(filter_opts) = &self.filter {
            build_opts.insert("filter".to_string(), filter_opts.to_string());
        }

        build_opts.insert("per_page".to_string(), self.per_page.to_string());
        build_opts.insert("page".to_string(), self.page.to_string());

        match Httpc::get(self.client, &url) {
            Ok(result) => {
                let response = result.into_json::<LogList>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }
}

impl<'a> LogManager<'a> {
    pub fn list(&self) -> LogListRequestBuilder<'a> {
        LogListRequestBuilder {
            client: self.client,
            page: 1,
            per_page: 100,
            sort: None,
            filter: None,
        }
    }
}
