use crate::client::{Auth, AuthSuccessResponse, Client, NoAuth};
use crate::httpc::Httpc;
use anyhow::{anyhow, Result};
use serde_json::json;

pub struct SuperUsersManager<'a> {
    pub client: &'a Client<NoAuth>,
}

impl<'a> SuperUsersManager<'a> {
    pub fn auth_with_password(&self, identity: &str, password: &str) -> Result<Client<Auth>> {
        let url = format!(
            "{}/api/collections/_superusers/auth-with-password",
            self.client.base_url
        );
        let credentials = json!({
            "identity": identity,
            "password": password,
        });
        match Httpc::post(self.client, &url, credentials.to_string()) {
            Ok(response) => match response.into_json::<AuthSuccessResponse>() {
                Ok(AuthSuccessResponse { token }) => Ok(Client {
                    base_url: self.client.base_url.clone(),
                    state: Auth,
                    auth_token: Some(token),
                }),
                Err(e) => Err(anyhow!("{}", e)),
            },
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
}
