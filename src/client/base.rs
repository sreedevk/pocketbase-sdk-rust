use reqwest::{Response, header};
use serde::Serialize;
use super::Client;
use std::error;
use serde::de::DeserializeOwned;

impl Client {
    pub async fn get<T: DeserializeOwned>(&self, path: String) -> Result<T, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                match reqwest::get(request_url).await {
                    Ok(response) => Ok(response.json::<T>().await?),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
                }
            },
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }

    pub async fn post<T: Serialize + Sized>(
        &self,
        path:
        String, body: &T
    ) -> Result<Response, Box<dyn error::Error>>
    {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::Client::new();
                let req = req_client
                    .post(request_url)
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
