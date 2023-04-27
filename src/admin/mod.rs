use crate::client::Auth;
use crate::client::Client;
use crate::errors::AuthError;
use crate::httpc::Httpc;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Admin;

#[derive(Debug, Clone, Deserialize)]
struct AuthSuccessResponse {
    token: String,
}

impl Admin {
    pub fn new(base_url: &str, identifier: &str, secret: &str) -> Result<Client<Auth>> {
        let url = format!("{}/api/admins/auth-with-password", base_url);
        let mut req_body: HashMap<String, String> = HashMap::new();
        req_body.insert("identity".to_string(), identifier.to_string());
        req_body.insert("password".to_string(), secret.to_string());

        let client = Client::new(base_url);
        match Httpc::post(&client, &url, req_body) {
            Ok(response) => {
                let raw_response = response.into_json::<AuthSuccessResponse>();
                match raw_response {
                    Ok(AuthSuccessResponse { token }) => Ok(Client {
                        base_url: base_url.to_string(),
                        state: Auth,
                        auth_token: Some(token),
                    }),
                    Err(_) => Err(AuthError::AuthResponseParseFailed.into()),
                }
            }
            Err(_) => Err(AuthError::AuthenticationFailed.into()),
        }
    }
}
