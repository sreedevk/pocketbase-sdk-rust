use std::collections::HashMap;

use anyhow::Result;
use log::{error, info};
use serde::Deserialize;
use thiserror::Error;
use ureq::Response;

use crate::httpc::Httpc;

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
pub struct AuthInfo {
    collection: String,
    identifier: String,
    secret: String,
}

impl AuthInfo {
    pub fn new(collection: &str, identifier: &str, secret: &str) -> Self {
        AuthInfo {
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
pub struct Client {
    pub base_url: String,
    pub auth_token: Option<String>,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
        }
    }

    pub fn authenticate_with_password(&mut self, auth_info: AuthInfo) -> Result<(), AuthError> {
        let url = format!(
            "{}/api/collections/{}/auth-with-password",
            self.base_url, auth_info.collection
        );

        match Httpc::post(&self, &url, auth_info.to_request_body()) {
            Ok(response) => {
                let raw_response = response.into_json::<AuthSuccessResponse>();
                match raw_response {
                    Ok(AuthSuccessResponse { token }) => {
                        self.auth_token = Some(token);
                        info!("authentication successful!");
                        Ok(())
                    }
                    Err(_) => {
                        error!(
                            "authentication parsing failed! response: {:?}",
                            raw_response
                        );
                        Err(AuthError::AuthResponseParseFailed)
                    }
                }
            }
            Err(e) => {
                error!("authentication failed! error: {:?}", e);
                Err(AuthError::AuthenticationFailed)
            }
        }
    }
}
