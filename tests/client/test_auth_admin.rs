use httpmock::prelude::*;
use pocketbase_sdk::client::Client;
use pocketbase_sdk::user::UserTypes;
use serde_json::json;

#[tokio::test]
async fn authenticate_admin() {
    let mockserver = mock_admin_login();
    let mut client = Client::new(mockserver.url("/api/").as_str()).unwrap();
    let auth = client
        .auth_via_email("sreedev@icloud.com", "Admin@123", UserTypes::Admin)
        .await;

    assert!(auth.is_ok());
}

fn mock_admin_login() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when
            .method(POST)
            .path("/api/admins/auth-via-email");

        then
            .status(200)
            .header("content-type", "application/json")
            .json_body(
                json!(
                    {
                        "admin": {
                            "id": "1n2b67cbuq8h2ei",
                            "created": "2022-10-05 03:16:44.732",
                            "updated": "2022-10-05 04:55:30.408",
                            "email": "sreedevpadmakumar@gmail.com",
                            "lastResetSentAt": "",
                            "avatar": 3
                        },
                        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NjY0NzQwMTQsImlkIjoiMW4yYjY3Y2J1cThoMmVpIiwidHlwZSI6ImFkbWluIn0.CTwSudbKGIfOkFv30FZJzqbiSltyKNaTrwiqZ5Hk0Lk"
                    }
                )
            );
    });

    server
}
