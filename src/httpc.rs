use std::collections::HashMap;
use ureq::{Request, Response};

use crate::client::Client;
use anyhow::Result;

pub struct Httpc;

impl Httpc {
    fn attach_auth_info(partial_request: Request, client: &Client) -> Result<Request> {
        match client.auth_token.as_ref() {
            Some(token) => Ok(partial_request.set("Authorization", token)),
            None => Ok(partial_request),
        }
    }

    pub fn get(client: &Client, url: &str) -> Result<Response> {
        Ok(ureq::get(url))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.call()?))
    }

    pub fn post(
        client: &Client,
        url: &str,
        body_content: HashMap<&str, &str>,
    ) -> Result<Response> {
        Ok(ureq::post(url))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.send_json(body_content)?))
    }
}
