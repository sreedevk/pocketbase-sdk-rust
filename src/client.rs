use crate::{collections::CollectionsManager, httpc::Httpc};
use crate::{logs::LogsManager, records::RecordsManager};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct AuthSuccessResponse {
    token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Credentials {
    collection: String,
    identifier: String,
    secret: String,
}

impl Credentials {
    pub fn new(collection: &str, identifier: &str, secret: &str) -> Self {
        Credentials {
            collection: collection.to_string(),
            identifier: identifier.to_string(),
            secret: secret.to_string(),
        }
    }

    pub fn to_request_body(&self) -> Result<String, serde_json::Error> {
        let mut body = HashMap::new();
        body.insert("identity".to_string(), self.identifier.clone());
        body.insert("password".to_string(), self.secret.clone());

        serde_json::to_string(&body)
    }
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

impl Client<Auth> {
    pub fn collections(&self) -> CollectionsManager {
        CollectionsManager { client: self }
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
        }
    }

    pub fn authenticate_with_password(&self, auth_info: Credentials) -> Result<Client<Auth>> {
        let url = format!(
            "{}/api/collections/{}/auth-with-password",
            self.base_url, auth_info.collection
        );

        let auth_payload = json!({
            "identity": auth_info.identifier,
            "password": auth_info.secret
        });

        match Httpc::post(self, &url, auth_payload.to_string()) {
            Ok(response) => {
                let raw_response = response.into_json::<AuthSuccessResponse>();
                match raw_response {
                    Ok(AuthSuccessResponse { token }) => Ok(Client {
                        base_url: self.base_url.clone(),
                        state: Auth,
                        auth_token: Some(token),
                    }),
                    Err(e) => Err(anyhow!("{}", e)),
                }
            }
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
}
