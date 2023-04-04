use super::Client;
use crate::user::{User, UserTypes};
use serde::Deserialize;
use std::collections::HashMap;
use surf::http::Method;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum PocketBaseAuthenticationError {
    #[error("Authentication Failed")]
    Unknown,
    #[error("You are not allowed to perform this request.")]
    NotAllowed,
    #[error("Failed to create record.")]
    FailedToCreateAccount,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuccessAuthResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FailureAuthResponse {
    pub code: String,
    pub message: String,
    pub data: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuccessRegisteredResponse {
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FailureRegisteredResponse {
    pub code: i32,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AuthResponse {
    SuccessAuthResponse(SuccessAuthResponse),
    FailureAuthResponse(FailureAuthResponse),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AuthRegisterResponse {
    Success(SuccessRegisteredResponse),
    Failure(FailureRegisteredResponse),
}

impl Client {
    pub async fn register_user<'a>(
        &mut self,
        email: &str,
        password: &str,
        username: Option<&str>,
    ) -> Result<SuccessRegisteredResponse, PocketBaseAuthenticationError> {
        let mut credentials: HashMap<String, String> = HashMap::from([
            (String::from("email"), email.to_string()),
            (String::from("password"), password.to_string()),
            (String::from("passwordConfirm"), password.to_string()),
        ]);
        if let Some(name) = username {
            credentials.insert(String::from("username"), name.to_string());
        }
        let request_url = self.base_url.join("collections/users/records");
        if request_url.is_err() {
            eprintln!("Request url is broken!");
            return Err(PocketBaseAuthenticationError::Unknown);
        }
        let request = surf::RequestBuilder::new(Method::Post, request_url.unwrap())
            .header("content-type", "application/json")
            .body_json(&credentials)
            .unwrap()
            .build();
        let http_client = surf::client();
        match http_client.recv_json::<AuthRegisterResponse>(request).await {
            Ok(resp) => match resp {
                AuthRegisterResponse::Success(account) => Ok(account),
                AuthRegisterResponse::Failure(fail) => Err(match fail.code {
                    400 => PocketBaseAuthenticationError::FailedToCreateAccount,
                    403 => PocketBaseAuthenticationError::NotAllowed,
                    _ => PocketBaseAuthenticationError::Unknown,
                }),
            },
            Err(error) => {
                eprintln!("Request failed! {}", error);
                let err = match error.status() {
                    surf::StatusCode::BadRequest => {
                        PocketBaseAuthenticationError::FailedToCreateAccount
                    }
                    surf::StatusCode::Forbidden => PocketBaseAuthenticationError::NotAllowed,
                    _ => PocketBaseAuthenticationError::Unknown,
                };
                Err(err)
            }
        }
    }

    pub async fn auth_via_email<'a>(
        &mut self,
        email: &str,
        password: &str,
        usertype: UserTypes,
    ) -> Result<(), PocketBaseAuthenticationError> {
        let credentials: HashMap<String, String> = HashMap::from([
            (String::from("identity"), email.to_string()),
            (String::from("password"), password.to_string()),
        ]);

        match usertype {
            UserTypes::User => self.authenticate_user(&credentials).await,
            UserTypes::Admin => self.authenticate_admin(&credentials).await,
        }
    }

    async fn authenticate_user(
        &mut self,
        credentials: &HashMap<String, String>,
    ) -> Result<(), PocketBaseAuthenticationError> {
        let request = self
            .post(
                "collections/users/auth-with-password",
                &credentials,
            )
            .await;
        let parsed_resp = match request {
            Ok(request) => {
                let http_client = surf::client();
                match http_client.recv_json(request).await {
                    Ok(resp) => Ok(resp),
                    Err(_err) => Err(PocketBaseAuthenticationError::Unknown),
                }
            }
            Err(_) => Err(PocketBaseAuthenticationError::Unknown),
        };

        match parsed_resp {
            Ok(body) => match body {
                AuthResponse::SuccessAuthResponse(response) => {
                    self.user = Some(User {
                        usertype: UserTypes::User,
                        token: response.token,
                    });

                    Ok(())
                }
                AuthResponse::FailureAuthResponse(_response) => {
                    Err(PocketBaseAuthenticationError::Unknown)
                }
            },
            Err(err) => Err(err),
        }
    }

    async fn authenticate_admin(
        &mut self,
        credentials: &HashMap<String, String>,
    ) -> Result<(), PocketBaseAuthenticationError> {
        let auth_request = self
            .post("admins/auth-via-email", &credentials)
            .await;
        let parsed_resp = match auth_request {
            Ok(request) => {
                let http_client = surf::client();
                match http_client.recv_json(request).await {
                    Ok(resp) => Ok(resp),
                    Err(_) => Err(super::base::PocketbaseClientError::InvalidRequest),
                }
            }
            Err(_) => Err(super::base::PocketbaseClientError::InvalidRequest),
        };

        match parsed_resp {
            Ok(body) => match body {
                AuthResponse::SuccessAuthResponse(response) => {
                    self.user = Some(User {
                        usertype: UserTypes::Admin,
                        token: response.token,
                    });

                    Ok(())
                }
                AuthResponse::FailureAuthResponse(_response) => {
                    Err(PocketBaseAuthenticationError::Unknown)
                }
            },
            Err(_) => Err(PocketBaseAuthenticationError::Unknown),
        }
    }
}
