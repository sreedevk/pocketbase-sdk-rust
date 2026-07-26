use httpmock::prelude::*;
use pocketbase_sdk::client::{Auth, Client};
use serde_json::json;

const TEST_TOKEN: &str = "eyJhbGciOiJIUzI1NiJ9.testtoken.sig";

#[test]
fn auth_with_password_via_manager() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/auth-with-password")
            .json_body(json!({ "identity": "sreedev@icloud.com", "password": "Sreedev123" }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": TEST_TOKEN, "record": { "id": "u1" } }));
    });

    let client = Client::new(server.base_url().as_str())
        .auth("users")
        .with_password("sreedev@icloud.com", "Sreedev123");
    assert!(client.is_ok());
}

#[test]
fn request_otp_returns_id() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/request-otp")
            .json_body(json!({ "email": "user@example.com" }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "otpId": "otp_abc123" }));
    });

    let otp = Client::new(server.base_url().as_str())
        .auth("users")
        .request_otp("user@example.com")
        .unwrap();
    assert_eq!(otp.otp_id, "otp_abc123");
}

#[test]
fn auth_with_otp_returns_client() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/auth-with-otp")
            .json_body(json!({ "otpId": "otp_abc123", "password": "123456" }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": TEST_TOKEN, "record": { "id": "u1" } }));
    });

    let client = Client::new(server.base_url().as_str())
        .auth("users")
        .with_otp("otp_abc123", "123456");
    assert_eq!(client.unwrap().auth_token.as_deref(), Some(TEST_TOKEN));
}

#[test]
fn request_password_reset_ok() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/request-password-reset")
            .json_body(json!({ "email": "user@example.com" }));
        then.status(204);
    });

    let result = Client::new(server.base_url().as_str())
        .auth("users")
        .request_password_reset("user@example.com");
    assert!(result.is_ok());
}

#[test]
fn confirm_password_reset_ok() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/confirm-password-reset")
            .json_body(json!({
                "token": "reset_tok",
                "password": "newpass",
                "passwordConfirm": "newpass"
            }));
        then.status(204);
    });

    let result = Client::new(server.base_url().as_str())
        .auth("users")
        .confirm_password_reset("reset_tok", "newpass", "newpass");
    assert!(result.is_ok());
}

#[test]
fn request_verification_ok() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/request-verification")
            .json_body(json!({ "email": "user@example.com" }));
        then.status(204);
    });

    let result = Client::new(server.base_url().as_str())
        .auth("users")
        .request_verification("user@example.com");
    assert!(result.is_ok());
}

#[test]
fn list_methods_parses_config() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/collections/users/auth-methods");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "mfa": { "enabled": false, "duration": 0 },
                "otp": { "enabled": true, "duration": 180 },
                "password": { "enabled": true, "identityFields": ["email", "username"] },
                "oauth2": {
                    "enabled": true,
                    "providers": [
                        { "name": "google", "displayName": "Google", "state": "abc", "authURL": "https://accounts.google.com/..." }
                    ]
                }
            }));
    });

    let methods = Client::new(server.base_url().as_str())
        .auth("users")
        .list_methods()
        .unwrap();
    assert!(methods.password.enabled);
    assert_eq!(methods.password.identity_fields, vec!["email", "username"]);
    assert!(methods.oauth2.enabled);
    assert_eq!(methods.oauth2.providers[0].name, "google");
    assert_eq!(methods.oauth2.providers[0].auth_url, "https://accounts.google.com/...");
}

#[test]
fn auth_with_oauth2_returns_client() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/auth-with-oauth2")
            .json_body(json!({
                "provider": "google",
                "code": "authcode",
                "codeVerifier": "verifier",
                "redirectUrl": "https://app.example.com/redirect"
            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": TEST_TOKEN, "record": { "id": "u1" } }));
    });

    let client = Client::new(server.base_url().as_str())
        .auth("users")
        .with_oauth2("google", "authcode", "verifier", "https://app.example.com/redirect");
    assert_eq!(client.unwrap().auth_token.as_deref(), Some(TEST_TOKEN));
}

fn authed_client(server: &MockServer) -> Client<Auth> {
    server.mock(|when, then| {
        when.method(POST).path("/api/collections/users/auth-with-password");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": TEST_TOKEN, "record": { "id": "u1" } }));
    });
    Client::new(server.base_url().as_str())
        .auth("users")
        .with_password("id", "pw")
        .unwrap()
}

#[test]
fn auth_refresh_returns_client() {
    let server = MockServer::start();
    let client = authed_client(&server);
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/auth-refresh")
            .header("Authorization", TEST_TOKEN);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": "refreshed.token.sig", "record": { "id": "u1" } }));
    });

    let refreshed = client.auth("users").refresh();
    assert_eq!(refreshed.unwrap().auth_token.as_deref(), Some("refreshed.token.sig"));
}

#[test]
fn request_email_change_ok() {
    let server = MockServer::start();
    let client = authed_client(&server);
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/request-email-change")
            .header("Authorization", TEST_TOKEN)
            .json_body(json!({ "newEmail": "new@example.com" }));
        then.status(204);
    });

    let result = client.auth("users").request_email_change("new@example.com");
    assert!(result.is_ok());
}

#[test]
fn impersonate_returns_client() {
    let server = MockServer::start();
    let client = authed_client(&server);
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/impersonate/u2")
            .header("Authorization", TEST_TOKEN)
            .json_body(json!({ "duration": 3600 }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": "impersonated.token.sig", "record": { "id": "u2" } }));
    });

    let impersonated = client.auth("users").impersonate("u2", Some(3600));
    assert_eq!(impersonated.unwrap().auth_token.as_deref(), Some("impersonated.token.sig"));
}
