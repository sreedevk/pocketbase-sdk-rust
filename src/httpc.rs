use ureq::{Request, Response};

use crate::client::Client;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Httpc {
    agent: ureq::Agent,
}

impl Httpc {
    pub fn new() -> Httpc {
        Httpc {
            agent: ureq::AgentBuilder::new()
                .build(),
        }
    }
    fn attach_auth_info<T>(partial_request: Request, client: &Client<T>) -> Result<Request> {
        match client.auth_token.as_ref() {
            Some(token) => Ok(partial_request.set("Authorization", token)),
            None => Ok(partial_request),
        }
    }

    pub fn get<T>(
        &self,
        client: &Client<T>,
        url: &str,
        query_params: Option<Vec<(&str, &str)>>,
    ) -> Result<Response> {
        Ok(self.agent.get(url))
            .and_then(|request| Self::attach_auth_info(request, client))
            .map(|request| {
                if let Some(pairs) = query_params {
                    request.query_pairs(pairs)
                } else {
                    request
                }
            })
            .and_then(|request| Ok(request.call()?))
    }

    pub fn post<T>(&self, client: &Client<T>, url: &str, body_content: String) -> Result<Response> {
        Ok(self.agent.post(url))
            .map(|request| request.set("Content-Type", "application/json"))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.send_string(body_content.as_str())?))
    }

    pub fn delete<T>(&self, client: &Client<T>, url: &str) -> Result<Response> {
        Ok(self.agent.delete(url))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.call()?))
    }

    pub fn patch<T>(
        &self,
        client: &Client<T>,
        url: &str,
        body_content: String,
    ) -> Result<Response> {
        Ok(self.agent.patch(url))
            .map(|request| request.set("Content-Type", "application/json"))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.send_string(body_content.as_str())?))
    }
}
