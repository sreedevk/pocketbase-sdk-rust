use serde::Deserialize;
use std::collections::HashMap;
use crate::user::{UserTypes, User};
use super::Client;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum PocketBaseAuthenticationError {
    #[error("Authentication Failed")]
    Unknown
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuccessAuthResponse {
    pub token: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FailureAuthResponse {
    pub code: String,
    pub message: String,
    pub data: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AuthResponse {
    SuccessAuthResponse(SuccessAuthResponse),
    FailureAuthResponse(FailureAuthResponse)
}

impl Client {
    pub async fn auth_via_email<'a>(
        &mut self,
        email: String, password: String,
        usertype: UserTypes
    ) -> Result<(), PocketBaseAuthenticationError>
    {
        let mut credentials: HashMap<String, String> = HashMap::new();
        credentials.insert(String::from("email"), email);
        credentials.insert(String::from("password"), password);

        match usertype {
            UserTypes::User => self.authenticate_user(&credentials).await,
            UserTypes::Admin => self.authenticate_admin(&credentials).await,
        }
    }

    async fn authenticate_user(&mut self, credentials: &HashMap<String, String>) -> Result<(), PocketBaseAuthenticationError> {
        let request = self.post(String::from("users/auth-via-email"), &credentials).await;
        let parsed_resp   = match request {
            Ok(request) => {
                let http_client = surf::client();
                match http_client.recv_json(request).await {
                    Ok(resp) => Ok(resp),
                    Err(_err) => Err(PocketBaseAuthenticationError::Unknown)
                }
            },
            Err(_) => Err(PocketBaseAuthenticationError::Unknown)
        };

        match parsed_resp {
            Ok(body) => {
                match body {
                    AuthResponse::SuccessAuthResponse(response) =>  {
                        self.user = Some(
                            User {
                                usertype: UserTypes::User,
                                token: response.token
                            }
                        );

                        Ok(())
                    },
                    AuthResponse::FailureAuthResponse(_response) => {
                        Err(PocketBaseAuthenticationError::Unknown)
                    }
                }
            },
            Err(err) => Err(err)
        }

    }

    async fn authenticate_admin(&mut self, credentials: &HashMap<String, String>) -> Result<(), PocketBaseAuthenticationError> {
        let auth_request = self.post(String::from("admins/auth-via-email"), &credentials).await;
        let parsed_resp   = match auth_request {
            Ok(request) => {
                let http_client  = surf::client();
                match http_client.recv_json(request).await {
                    Ok(resp) => Ok(resp),
                    Err(_) => Err(super::base::PocketbaseClientError::InvalidRequest)
                }
            },
            Err(_) => Err(super::base::PocketbaseClientError::InvalidRequest)
        };

        match parsed_resp {
            Ok(body) => {
                match body {
                    AuthResponse::SuccessAuthResponse(response) =>  {
                        self.user = Some(
                            User {
                                usertype: UserTypes::Admin,
                                token: response.token
                            }
                        );

                        Ok(())
                    },
                    AuthResponse::FailureAuthResponse(_response) => {
                        Err(PocketBaseAuthenticationError::Unknown)
                    }
                }
            },
            Err(_) => Err(PocketBaseAuthenticationError::Unknown)
        }
    }
}
