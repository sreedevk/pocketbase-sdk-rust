use pocketbase_sdk::client::SyncClient;
use pocketbase_sdk::user::UserTypes;
use httpmock::prelude::*;
use serde_json::json;

#[test]
fn authenticate_user() {
    let mockserver = mock_user_login();
    let mut client = SyncClient::new(mockserver.url("/api/").as_str()).unwrap();
    let auth       = client.auth_via_email(
        String::from("sreedev@icloud.com"),
        String::from("Admin@123"),
        UserTypes::User
    );

    assert!(auth.is_ok());
}

fn mock_user_login() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when
            .method(POST)
            .path("/api/users/auth-via-email");

        then
            .status(200)
            .header("content-type", "application/json")
            .json_body(
                json!({
                    "user": {
                        "id": "1n2b67cbuq8h2ei",
                        "created": "2022-10-05 03:16:44.732",
                        "updated": "2022-10-05 04:55:30.408",
                        "email": "sreedevpadmakumar@gmail.com",
                        "lastResetSentAt": "",
                        "avatar": 3
                    },
                    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NjY0NzQwMTQsImlkIjoiMW4yYjY3Y2J1cThoMmVpIiwidHlwZSI6ImFkbWluIn0.CTwSudbKGIfOkFv30FZJzqbiSltyKNaTrwiqZ5Hk0Lk"
                })
            );
    });

    server
}


