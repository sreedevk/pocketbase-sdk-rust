use reqwest::{Response, blocking, header::{self, AUTHORIZATION}};
use serde::Serialize;
use super::{Client, SyncClient};
use std::error;
use crate::user::UserTypes;

impl Client {
    pub async fn get(&self, path: String, params: Option<&[(&str, &str)]>) -> 
    Result<Response, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::Client::new();
                let request = req_client.get(request_url);
                let opts_attached = match params {
                    Some(args) => request.query(args),
                    None => request
                }; 

                let authed_req = match &self.user {
                    Some(user) => {
                        match &user.usertype {
                            UserTypes::User => {
                                opts_attached.header(
                                    AUTHORIZATION,
                                    format!("User {}", user.token)
                                )
                            },
                            UserTypes::Admin => {
                                opts_attached.header(
                                    AUTHORIZATION,
                                    format!("Admin {}", user.token)
                                )

                            }
                        }
                    }
                    None => opts_attached
                };

                match authed_req.send().await {
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

    pub async fn delete(
        &self,
        path: String,
        params: Option<&[(&str, &str)]>
    ) -> Result<Response, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::Client::new();
                let req = req_client
                    .patch(request_url)
                    .header(header::CONTENT_TYPE, "application/json");

                let opts_attached = match params {
                    Some(args) => req.query(args),
                    None => req
                };

                let authed_req = match &self.user {
                    Some(user) => {
                        match &user.usertype {
                            UserTypes::User => {
                                opts_attached.header(
                                    AUTHORIZATION,
                                    format!("User {}", user.token)
                                )
                            },
                            UserTypes::Admin => {
                                opts_attached.header(
                                    AUTHORIZATION,
                                    format!("Admin {}", user.token)
                                )

                            }
                        }
                    }
                    None => opts_attached
                };

                match authed_req.send().await {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }
}

impl SyncClient {
    pub fn get(&self, path: String, params: Option<&[(&str, &str)]>) -> 
    Result<blocking::Response, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::blocking::Client::new();
                let request = req_client.get(request_url);
                let opts_attached = match params {
                    Some(args) => request.query(args),
                    None => request
                }; 

                let authed_req = match &self.user {
                    Some(user) => {
                        match &user.usertype {
                            UserTypes::User => {
                                opts_attached.header(
                                    AUTHORIZATION,
                                    format!("User {}", user.token)
                                )
                            },
                            UserTypes::Admin => {
                                opts_attached.header(
                                    AUTHORIZATION,
                                    format!("Admin {}", user.token)
                                )

                            }
                        }
                    }
                    None => opts_attached
                };

                match authed_req.send() {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }

    pub fn post<T: Serialize + Sized>(
        &self,
        path: String,
        body: &T
    ) -> Result<blocking::Response, Box<dyn error::Error>>
    {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::blocking::Client::new();
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

                match authed_req.send() {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }

    pub fn patch<T: Serialize + Sized>(
        &self,
        path: String,
        body: &T
    ) -> Result<blocking::Response, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::blocking::Client::new();
                let req = req_client
                    .patch(request_url)
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

                match authed_req.send() {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }

    pub fn delete(
        &self,
        path: String,
        params: Option<&[(&str, &str)]>
    ) -> Result<blocking::Response, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::blocking::Client::new();
                let req = req_client
                    .patch(request_url)
                    .header(header::CONTENT_TYPE, "application/json");

                let opts_attached = match params {
                    Some(args) => req.query(args),
                    None => req
                };

                let authed_req = match &self.user {
                    Some(user) => {
                        match &user.usertype {
                            UserTypes::User => {
                                opts_attached.header(
                                    AUTHORIZATION,
                                    format!("User {}", user.token)
                                )
                            },
                            UserTypes::Admin => {
                                opts_attached.header(
                                    AUTHORIZATION,
                                    format!("Admin {}", user.token)
                                )

                            }
                        }
                    }
                    None => opts_attached
                };

                match authed_req.send() {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }
}
