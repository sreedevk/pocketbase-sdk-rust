use std::collections::HashMap;

use anyhow::Result;
use log::{error, info};
use serde::Deserialize;
use thiserror::Error;
use ureq::Response;

use crate::{httpc::Httpc, collections::CollectionsManager};

#[derive(Debug, Deserialize)]
struct AuthSuccessResponse {
    token: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication Failed")]
    AuthenticationFailed,
    #[error("Auth Response Parse Error")]
    AuthResponseParseFailed,
}

#[derive(Debug, Clone)]
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

    pub fn to_request_body(&self) -> HashMap<&str, &str> {
        let mut body = HashMap::new();
        body.insert("identity", self.identifier.as_str());
        body.insert("password", self.secret.as_str());

        body
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
}

impl Client<NoAuth> {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
            state: NoAuth,
        }
    }

    pub fn authenticate_with_password(
        &self,
        auth_info: Credentials,
    ) -> Result<Client<Auth>, AuthError> {
        let url = format!(
            "{}/api/collections/{}/auth-with-password",
            self.base_url, auth_info.collection
        );

        match Httpc::post(self, &url, auth_info.to_request_body()) {
            Ok(response) => {
                let raw_response = response.into_json::<AuthSuccessResponse>();
                match raw_response {
                    Ok(AuthSuccessResponse { token }) => Ok(Client {
                        base_url: self.base_url.clone(),
                        state: Auth,
                        auth_token: Some(token),
                    }),
                    Err(_) => Err(AuthError::AuthResponseParseFailed),
                }
            }
            Err(e) => Err(AuthError::AuthenticationFailed),
        }
    }
}
