use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

use crate::{users::{AuthenticatedUser, User}, Client};

#[derive(Serialize)]
pub struct Changeset<'a, T> {
    pub user:  &'a User<'a>,
    pub resource: &'a str,
    pub record: &'a T
}

impl<'a, T: Serialize> Changeset<'a, T> {
    pub async fn insert(client: &Client, changeset: Changeset<'a, T>) -> Result<HashMap<String, String>, Box< dyn Error>> {
        let url = format!("collections/{}/records", changeset.resource);
        let create_res = client.post(url, &changeset.record).await;
        match create_res {
            Ok(response) => Ok(response.json::<HashMap<String,String>>().await.unwrap()),
            Err(e) => Err(e)
        }
    }
}
