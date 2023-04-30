use crate::client::Auth;
use crate::client::Client;
use crate::httpc::Httpc;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::json;

pub struct Admin;

#[derive(Debug, Clone, Deserialize)]
struct AuthSuccessResponse {
    token: String,
}

impl Admin {
    pub fn new(base_url: &str, identifier: &str, secret: &str) -> Result<Client<Auth>> {
        let url = format!("{}/api/admins/auth-with-password", base_url);
        let credentials = json!({
            "identity": identifier,
            "password": secret,
        });
        let client = Client::new(base_url);
        match Httpc::post(&client, &url, credentials.to_string()) {
            Ok(response) => {
                let raw_response = response.into_json::<AuthSuccessResponse>();
                match raw_response {
                    Ok(AuthSuccessResponse { token }) => Ok(Client {
                        base_url: base_url.to_string(),
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
