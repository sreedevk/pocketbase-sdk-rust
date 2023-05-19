use crate::{collections::CollectionsManager, httpc::Httpc};
use crate::{logs::LogsManager, records::RecordsManager};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct AuthSuccessResponse {
    token: String,
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
    pub httpc: Httpc,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthCheckResponse {
    pub code: i32,
    pub message: String,
}

impl Client<Auth> {
    pub fn collections(&self) -> CollectionsManager {
        CollectionsManager { client: self }
    }

    pub fn health_check(&self) -> Result<HealthCheckResponse> {
        let url = format!("{}/api/health", self.base_url);
        match self.httpc.get(self, &url, None) {
            Ok(response) => Ok(response.into_json::<HealthCheckResponse>()?),
            Err(e) => Err(anyhow!("{}", e))
        }
    }

    pub fn logs(&self) -> LogsManager {
        LogsManager { client: self }
    }

    pub fn records(&self, record_name: &'static str) -> RecordsManager {
        RecordsManager {
            client: self,
            name: record_name,
        }
    }
}

impl Client<NoAuth> {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
            state: NoAuth,
            httpc: Httpc::new(),
        }
    }

    pub fn health_check(&self) -> Result<HealthCheckResponse> {
        let url = format!("{}/api/health", self.base_url);
        match self.httpc.get(self, &url, None) {
            Ok(response) => Ok(response.into_json::<HealthCheckResponse>()?),
            Err(e) => Err(anyhow!("{}", e))
        }
    }

    pub fn auth_with_password(&self, collection: &str, identifier: &str, secret: &str) -> Result<Client<Auth>> {
        let url = format!(
            "{}/api/collections/{}/auth-with-password",
            self.base_url, collection
        );

        let auth_payload = json!({
            "identity": identifier,
            "password": secret
        });

        match self.httpc.post(self, &url, auth_payload.to_string()) {
            Ok(response) => {
                let raw_response = response.into_json::<AuthSuccessResponse>();
                match raw_response {
                    Ok(AuthSuccessResponse { token }) => Ok(Client {
                        base_url: self.base_url.clone(),
                        state: Auth,
                        auth_token: Some(token),
                        httpc: self.httpc.clone(),
                    }),
                    Err(e) => Err(anyhow!("{}", e)),
                }
            }
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
}
