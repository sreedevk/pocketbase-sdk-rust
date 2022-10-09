mod auth;
mod base;

use std::error;
use url::Url;
use crate::user::User;

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    user: Option<User>
}

impl Client {
    pub fn new<'a>(raw_url: &'a str) -> Result<Client, Box<dyn error::Error>> {
        match Url::parse(raw_url) {
            Ok(url_object) => Ok(Client { base_url: url_object, user: None }) ,
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>)
        }
    }
}
