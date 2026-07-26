use crate::client::{Auth, AuthSuccessResponse, Client, NoAuth};
use crate::httpc::Httpc;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::json;

pub struct AuthManager<'a, State> {
    pub client: &'a Client<State>,
    pub collection: &'a str,
}

impl<'a, State> AuthManager<'a, State> {
    fn endpoint(&self, suffix: &str) -> String {
        format!(
            "{}/api/collections/{}/{}",
            self.client.base_url, self.collection, suffix
        )
    }

    fn post_for_token(&self, suffix: &str, payload: serde_json::Value) -> Result<Client<Auth>> {
        let url = self.endpoint(suffix);
        match Httpc::post(self.client, &url, payload.to_string()) {
            Ok(response) => {
                let AuthSuccessResponse { token } = response.into_json::<AuthSuccessResponse>()?;
                Ok(Client::authenticated(self.client.base_url.clone(), token))
            }
            Err(e) => Err(anyhow!("{}", e)),
        }
    }

    fn post_expect_204(&self, suffix: &str, payload: serde_json::Value) -> Result<()> {
        let url = self.endpoint(suffix);
        match Httpc::post(self.client, &url, payload.to_string()) {
            Ok(response) => {
                if response.status() == 204 {
                    Ok(())
                } else {
                    Err(anyhow!("unexpected status: {}", response.status()))
                }
            }
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtpResponse {
    pub otp_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthMethodsResponse {
    pub password: PasswordAuthConfig,
    pub oauth2: OAuth2Config,
    pub mfa: MfaConfig,
    pub otp: OtpConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordAuthConfig {
    pub enabled: bool,
    #[serde(default)]
    pub identity_fields: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Config {
    pub enabled: bool,
    #[serde(default)]
    pub providers: Vec<OAuth2ProviderInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2ProviderInfo {
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub state: String,
    #[serde(default, rename = "authURL")]
    pub auth_url: String,
    #[serde(default)]
    pub code_verifier: String,
    #[serde(default)]
    pub code_challenge: String,
    #[serde(default)]
    pub code_challenge_method: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MfaConfig {
    pub enabled: bool,
    #[serde(default)]
    pub duration: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtpConfig {
    pub enabled: bool,
    #[serde(default)]
    pub duration: i64,
}

impl<'a> AuthManager<'a, Auth> {
    pub fn refresh(&self) -> Result<Client<Auth>> {
        self.post_for_token("auth-refresh", json!({}))
    }

    pub fn request_email_change(&self, new_email: &str) -> Result<()> {
        self.post_expect_204("request-email-change", json!({ "newEmail": new_email }))
    }

    pub fn impersonate(&self, record_id: &str, duration: Option<u64>) -> Result<Client<Auth>> {
        let payload = match duration {
            Some(seconds) => json!({ "duration": seconds }),
            None => json!({}),
        };
        self.post_for_token(&format!("impersonate/{}", record_id), payload)
    }
}

impl<'a> AuthManager<'a, NoAuth> {
    pub fn with_password(&self, identity: &str, password: &str) -> Result<Client<Auth>> {
        self.post_for_token(
            "auth-with-password",
            json!({ "identity": identity, "password": password }),
        )
    }

    pub fn request_otp(&self, email: &str) -> Result<OtpResponse> {
        let url = self.endpoint("request-otp");
        match Httpc::post(self.client, &url, json!({ "email": email }).to_string()) {
            Ok(response) => Ok(response.into_json::<OtpResponse>()?),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }

    pub fn with_otp(&self, otp_id: &str, password: &str) -> Result<Client<Auth>> {
        self.post_for_token(
            "auth-with-otp",
            json!({ "otpId": otp_id, "password": password }),
        )
    }

    pub fn request_verification(&self, email: &str) -> Result<()> {
        self.post_expect_204("request-verification", json!({ "email": email }))
    }

    pub fn confirm_verification(&self, token: &str) -> Result<()> {
        self.post_expect_204("confirm-verification", json!({ "token": token }))
    }

    pub fn request_password_reset(&self, email: &str) -> Result<()> {
        self.post_expect_204("request-password-reset", json!({ "email": email }))
    }

    pub fn confirm_password_reset(&self, token: &str, password: &str, password_confirm: &str) -> Result<()> {
        self.post_expect_204(
            "confirm-password-reset",
            json!({ "token": token, "password": password, "passwordConfirm": password_confirm }),
        )
    }

    pub fn confirm_email_change(&self, token: &str, password: &str) -> Result<()> {
        self.post_expect_204(
            "confirm-email-change",
            json!({ "token": token, "password": password }),
        )
    }

    pub fn list_methods(&self) -> Result<AuthMethodsResponse> {
        let url = self.endpoint("auth-methods");
        match Httpc::get(self.client, &url, None) {
            Ok(response) => Ok(response.into_json::<AuthMethodsResponse>()?),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }

    pub fn with_oauth2(&self, provider: &str, code: &str, code_verifier: &str, redirect_url: &str) -> Result<Client<Auth>> {
        self.post_for_token(
            "auth-with-oauth2",
            json!({
                "provider": provider,
                "code": code,
                "codeVerifier": code_verifier,
                "redirectUrl": redirect_url
            }),
        )
    }
}
