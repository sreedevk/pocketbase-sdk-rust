use serde::Serialize;
use std::collections::HashMap;
use super::changeset::*;
use crate::Client;
use std::error::Error;

pub async fn insert<'a, T: Serialize>(changeset: Changeset<'a, T>, client: &Client) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let url = format!("collections/{}/records", changeset.resource);
    let result = client.post(url, &changeset.record).await;
    match result {
        Ok(response) => Ok(response.json::<HashMap<String,String>>().await.unwrap()),
        Err(e) => Err(e)
    }
}

pub async fn update<'a, T: Serialize>(changeset: Changeset<'a, T>, client: &Client, id: &'a str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let url = format!("collections/{}/records/{}", changeset.resource, id);
    let result = client.patch(url, &changeset.record).await;
    match result {
        Ok(response) => Ok(response.json::<HashMap<String,String>>().await.unwrap()),
        Err(e) => Err(e)
    }
}
