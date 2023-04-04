use super::Client;
use crate::user::UserTypes;
use serde::Serialize;
use surf::{http::Method, Request};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PocketbaseClientError {
    #[error("Request Invalid")]
    InvalidRequest,
}

impl Client {
    pub async fn get<T: Serialize>(
        &self,
        path: &str,
        params: Option<&T>,
    ) -> Result<Request, PocketbaseClientError> {
        match self.base_url.join(path) {
            Ok(request_url) => {
                let request = surf::RequestBuilder::new(Method::Get, request_url);
                let opts_attached = match params {
                    Some(args) => request.query(args).unwrap(),
                    None => request,
                };

                let authed_req = match &self.user {
                    Some(user) => match &user.usertype {
                        UserTypes::User => {
                            opts_attached.header("Authorization", format!("{}", user.token))
                        }
                        UserTypes::Admin => {
                            opts_attached.header("Authorization", format!("{}", user.token))
                        }
                    },
                    None => opts_attached,
                };

                Ok(authed_req.build())
            }
            Err(_) => Err(PocketbaseClientError::InvalidRequest),
        }
    }

    pub async fn post<T: Serialize + Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Request, PocketbaseClientError> {
        match self.base_url.join(path) {
            Ok(request_url) => {
                let request = surf::RequestBuilder::new(Method::Post, request_url)
                    .header("content-type", "application/json")
                    .body_json(body)
                    .unwrap();

                let authed_req = match &self.user {
                    Some(user) => match &user.usertype {
                        UserTypes::User => {
                            request.header("Authorization", format!("User {}", user.token))
                        }
                        UserTypes::Admin => {
                            request.header("Authorization", format!("Admin {}", user.token))
                        }
                    },
                    None => request,
                };

                Ok(authed_req.build())
            }
            Err(_) => Err(PocketbaseClientError::InvalidRequest),
        }
    }

    pub async fn patch<T: Serialize + Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Request, PocketbaseClientError> {
        match self.base_url.join(path) {
            Ok(request_url) => {
                let request = surf::RequestBuilder::new(Method::Patch, request_url)
                    .header("content-type", "application/json")
                    .body_json(body)
                    .unwrap();

                let authed_req = match &self.user {
                    Some(user) => match &user.usertype {
                        UserTypes::User => {
                            request.header("Authorization", format!("User {}", user.token))
                        }
                        UserTypes::Admin => {
                            request.header("Authorization", format!("Admin {}", user.token))
                        }
                    },
                    None => request,
                };

                Ok(authed_req.build())
            }
            Err(_) => Err(PocketbaseClientError::InvalidRequest),
        }
    }

    pub async fn delete(&self, path: &str) -> Result<Request, PocketbaseClientError> {
        match self.base_url.join(path) {
            Ok(request_url) => {
                let request = surf::RequestBuilder::new(Method::Delete, request_url)
                    .header("content-type", "application/json");

                let authed_req = match &self.user {
                    Some(user) => match &user.usertype {
                        UserTypes::User => {
                            request.header("Authorization", format!("User {}", user.token))
                        }
                        UserTypes::Admin => {
                            request.header("Authorization", format!("Admin {}", user.token))
                        }
                    },
                    None => request,
                };

                Ok(authed_req.build())
            }
            Err(_) => Err(PocketbaseClientError::InvalidRequest),
        }
    }
}
