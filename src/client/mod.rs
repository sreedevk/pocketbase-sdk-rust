mod auth;
mod base;

use crate::user::User;
use std::error;
use url::Url;

#[derive(Debug)]
pub struct Client {
    pub base_url: Url,
    pub user: Option<User>,
}

impl Client {
    pub fn new<'a>(raw_url: &'a str) -> Result<Client, Box<dyn error::Error>> {
        match Url::parse(raw_url) {
            Ok(url_object) => Ok(Client {
                base_url: url_object,
                user: None,
            }),
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>),
        }
    }
}
