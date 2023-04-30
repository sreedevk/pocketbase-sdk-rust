use httpmock::prelude::*;
use pocketbase_sdk::client::Client;
use serde_json::json;

#[test]
pub fn authenticate_record_success() {
    let mockserver = mock_admin_login();
    let client = Client::new(mockserver.base_url().as_str()).auth_with_password(
        "users",
        "sreedev@icloud.com",
        "Sreedev123",
    );
    assert!(client.is_ok());
}

#[test]
pub fn authenticate_record_error() {
    let mockserver = mock_admin_login();
    let client = Client::new(mockserver.base_url().as_str()).auth_with_password(
        "users",
        "bingo",
        "bango",
    );
    assert!(client.is_err());
}

fn mock_admin_login() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .json_body(json!({
                "identity": "bingo",
                "password": "bango"
            }))
            .path("/api/collections/users/auth-with-password");

        then.status(400)
            .header("content-type", "application/json")
            .json_body(json!({
                    "code": 400,
                    "message": "An error occurred while submitting the form.",
                    "data": {
                    "password": {
                    "code": "validation_required",
                    "message": "Missing required value."
                }
            }
            }));
    });
    server.mock(|when, then| {
        when
            .method(POST)
            .json_body(json!({
                "identity": "sreedev@icloud.com",
                "password": "Sreedev123"
            }))
            .path("/api/collections/users/auth-with-password");

        then
            .status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                    "token": "eyJhbGciOiJIUzI1NiJ9.eyJpZCI6IjRxMXhsY2xtZmxva3UzMyIsInR5cGUiOiJhdXRoUmVjb3JkIiwiY29sbGVjdGlvbklkIjoiX3BiX3VzZXJzX2F1dGhfIiwiZXhwIjoyMjA4OTg1MjYxfQ.UwD8JvkbQtXpymT09d7J6fdA0aP9g4FJ1GPh_ggEkzc",
                    "record": {
                    "id": "8171022dc95a4ed",
                    "collectionId": "d2972397d45614e",
                    "collectionName": "users",
                    "created": "2022-06-24 06:24:18.434Z",
                    "updated": "2022-06-24 06:24:18.889Z",
                    "username": "test@example.com",
                    "email": "test@example.com",
                    "verified": false,
                    "emailVisibility": true,
                    "someCustomField": "example 123"
                }
            }));
    });

    server
}
