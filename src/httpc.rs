use std::collections::HashMap;
use ureq::{Request, Response};

use crate::client::{Client, Auth, NoAuth};
use anyhow::Result;

pub struct Httpc;

impl Httpc {
    fn attach_auth_info<T>(partial_request: Request, client: &Client<T>) -> Result<Request> {
        match client.auth_token.as_ref() {
            Some(token) => Ok(partial_request.set("Authorization", token)),
            None => Ok(partial_request),
        }
    }

    pub fn get(client: &Client<Auth>, url: &str) -> Result<Response> {
        Ok(ureq::get(url))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.call()?))
    }

    pub fn post<T>(
        client: &Client<T>,
        url: &str,
        body_content: HashMap<String, String>,
    ) -> Result<Response> {
        Ok(ureq::post(url))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.send_json(body_content)?))
    }
}