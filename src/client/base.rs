use reqwest::{Response, header::{self, AUTHORIZATION}};
use serde::Serialize;
use super::Client;
use std::error;
use crate::user::UserTypes;

impl Client {
    pub async fn get(&self, path: String) -> 
    Result<Response, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                match reqwest::get(request_url).await {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }

    pub async fn post<T: Serialize + Sized>(
        &self,
        path: String,
        body: &T
    ) -> Result<Response, Box<dyn error::Error>>
    {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::Client::new();
                let req = req_client
                    .post(request_url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(serde_json::to_string(body).unwrap());

                let authed_req = match &self.user {
                    Some(user) => {
                        match &user.usertype {
                            UserTypes::User => {
                                req.header(
                                    AUTHORIZATION,
                                    format!("User {}", user.token)
                                )
                            },
                            UserTypes::Admin => {
                                req.header(
                                    AUTHORIZATION,
                                    format!("Admin {}", user.token)
                                )

                            }
                        }
                    }
                    None => req
                };

                match authed_req.send().await {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }

    pub async fn patch<T: Serialize + Sized>(
        &self,
        path: String,
        body: &T
    ) -> Result<Response, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::Client::new();
                let req = req_client
                    .patch(request_url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(serde_json::to_string(body).unwrap());

                match req.send().await {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }
}
