use crate::client::{Auth, Client};
use crate::httpc::Httpc;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub struct LogsManager<'a> {
    pub client: &'a Client<Auth>,
}

#[derive(Debug, Clone)]
pub struct LogListRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub page: i32,
    pub per_page: i32,
    pub sort: Option<&'a str>,
    pub filter: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct LogViewRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub id: &'a str,
}

#[derive(Debug, Clone)]
pub struct LogStatisticsRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub filter: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogListItem {
    pub id: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub url: String,
    pub method: String,
    pub status: i32,
    pub ip: Option<String>,
    pub referer: String,
    pub user_agent: String,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogList {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i32,
    pub items: Vec<LogListItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogStatDataPoint {
    pub total: i32,
    pub date: String,
}

impl<'a> LogStatisticsRequestBuilder<'a> {
    pub fn filter(&self, filter_query: &'a str) -> Self {
        Self {
            filter: Some(filter_query),
            ..self.clone()
        }
    }

    pub fn call(&self) -> Result<Vec<LogStatDataPoint>> {
        let url = format!("{}/api/logs/requests/stats", self.client.base_url);
        let mut build_opts = Vec::new();
        if let Some(filter_opts) = &self.filter {
            build_opts.push(("filter", filter_opts.to_owned()));
        }

        match Httpc::get(self.client, &url, Some(build_opts)) {
            Ok(result) => {
                let response = result.into_json::<Vec<LogStatDataPoint>>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }
}

impl<'a> LogViewRequestBuilder<'a> {
    pub fn call(&self) -> Result<LogListItem> {
        let url = format!("{}/api/logs/requests/{}", self.client.base_url, self.id);
        match Httpc::get(self.client, &url, None) {
            Ok(result) => {
                let response = result.into_json::<LogListItem>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }
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
        LogListRequestBuilder {
            filter: Some(filter_opts),
            ..self.clone()
        }
    }

    pub fn sort(&self, sort_opts: &'a str) -> Self {
        LogListRequestBuilder {
            sort: Some(sort_opts),
            ..self.clone()
        }
    }

    pub fn call(&self) -> Result<LogList> {
        let url = format!("{}/api/logs/requests", self.client.base_url);
        let mut build_opts = Vec::new();

        if let Some(sort_opts) = &self.sort { build_opts.push(("sort", sort_opts.to_owned())) }
        if let Some(filter_opts) = &self.filter { build_opts.push(("filter", filter_opts.to_owned())) }
        let per_page_opts = self.per_page.to_string();
        let page_opts = self.page.to_string();
        build_opts.push(("per_page", per_page_opts.as_str()));
        build_opts.push(("page", page_opts.as_str()));

        match Httpc::get(self.client, &url, Some(build_opts)) {
            Ok(result) => {
                let response = result.into_json::<LogList>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }
}

impl<'a> LogsManager<'a> {
    pub fn list(&self) -> LogListRequestBuilder<'a> {
        LogListRequestBuilder {
            client: self.client,
            page: 1,
            per_page: 100,
            sort: None,
            filter: None,
        }
    }

    pub fn view(&self, id: &'a str) -> LogViewRequestBuilder<'a> {
        LogViewRequestBuilder {
            client: self.client,
            id,
        }
    }

    pub fn statistics(&self) -> LogStatisticsRequestBuilder<'a> {
        LogStatisticsRequestBuilder { client: self.client, filter: None } 
    }
}
