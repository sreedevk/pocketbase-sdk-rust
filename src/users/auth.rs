use serde::{Serialize, Deserialize};
use crate::Client;
use std::error::Error;
use std::collections::HashMap;
use reqwest::Response;

pub struct Auth;

#[derive(Debug, Serialize)]
pub struct User<'a>  {
    pub data: Option<&'a AuthenticatedUser>,
    pub token: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    id: String,
    avatar: String,
    created: String,
    name: String,
    updated: String,
    user_id: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
    pub id: String,
    created: String,
    updated: String,
    email: String,
    last_reset_sent_at: String,
    verified: bool,
    last_verification_sent_at: String,
    profile: UserProfile
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    user: AuthenticatedUser,
    token: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FailureResponse {
    code: String,
    message: String,
    data: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthParams {
    email: String,
    password: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AuthResponse {
    SuccessResponse {
        user: AuthenticatedUser,
        token: String
    },

    FailureResponse {
        message: String,
        data: HashMap<String, String>
    }
}

impl Auth {
    pub async fn via_email(email: String, password: String, client: &Client) -> Result<AuthResponse, Box<dyn Error>> {
        let auth_response: Result<Response, Box<dyn Error>> = client.post(
            String::from("users/auth-via-email"),
            &AuthParams { email, password }
        ).await;

        match auth_response {
            Ok(response) => Ok(response.json::<AuthResponse>().await.unwrap()),
            Err(e) => Err(e)
        }
    }

    pub async fn get_user(auth_response: &AuthResponse) -> User {
        match auth_response {
            AuthResponse::SuccessResponse { user, token } => {
                User {
                    data: Some(user),
                    token: Some(String::from(token))
                }
            },
            AuthResponse::FailureResponse { message, data} => {
                User { data: None, token: None}
            }
        }
    }
} 
