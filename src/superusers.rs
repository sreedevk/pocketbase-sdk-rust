use crate::client::{Auth, Client, NoAuth};
use anyhow::Result;

pub struct SuperUsersManager<'a> {
    pub client: &'a Client<NoAuth>,
}

impl<'a> SuperUsersManager<'a> {
    pub fn auth_with_password(&self, identity: &str, password: &str) -> Result<Client<Auth>> {
        self.client.auth("_superusers").with_password(identity, password)
    }
}
