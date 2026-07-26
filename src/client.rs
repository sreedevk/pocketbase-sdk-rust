use crate::auth::AuthManager;
use crate::superusers::SuperUsersManager;
use crate::{collections::CollectionsManager, httpc::Httpc};
use crate::{logs::LogsManager, records::RecordsManager};
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AuthSuccessResponse {
    pub(crate) token: String,
}

#[derive(Debug, Clone)]
pub struct NoAuth;

#[derive(Debug, Clone)]
pub struct Auth;

#[derive(Debug, Clone)]
pub struct Client<State = NoAuth> {
    pub base_url: String,
    pub auth_token: Option<String>,
    pub state: State,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthCheckResponse {
    pub code: i32,
    pub message: String,
}

impl Client<Auth> {
    pub(crate) fn authenticated(base_url: String, token: String) -> Self {
        Self {
            base_url,
            auth_token: Some(token),
            state: Auth,
        }
    }

    pub fn auth<'a>(&'a self, collection: &'a str) -> AuthManager<'a, Auth> {
        AuthManager { client: self, collection }
    }

    pub fn collections(&self) -> CollectionsManager<'_> {
        CollectionsManager { client: self }
    }

    pub fn health_check(&self) -> Result<HealthCheckResponse> {
        let url = format!("{}/api/health", self.base_url);
        match Httpc::get(self, &url, None) {
            Ok(response) => Ok(response.into_json::<HealthCheckResponse>()?),
            Err(e) => Err(anyhow!("{}", e))
        }
    }

    pub fn logs(&self) -> LogsManager<'_> {
        LogsManager { client: self }
    }

    pub fn records(&self, record_name: &'static str) -> RecordsManager<'_> {
        RecordsManager {
            client: self,
            name: record_name,
        }
    }
}

impl Client<NoAuth> {
    pub fn superusers(&self) -> SuperUsersManager<'_> {
        SuperUsersManager { client: self }
    }

    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
            state: NoAuth,
        }
    }

    pub fn health_check(&self) -> Result<HealthCheckResponse> {
        let url = format!("{}/api/health", self.base_url);
        match Httpc::get(self, &url, None) {
            Ok(response) => Ok(response.into_json::<HealthCheckResponse>()?),
            Err(e) => Err(anyhow!("{}", e))
        }
    }

    pub fn auth<'a>(&'a self, collection: &'a str) -> AuthManager<'a, NoAuth> {
        AuthManager { client: self, collection }
    }

    pub fn auth_with_password(&self, collection: &str, identifier: &str, secret: &str) -> Result<Client<Auth>> {
        self.auth(collection).with_password(identifier, secret)
    }
}
