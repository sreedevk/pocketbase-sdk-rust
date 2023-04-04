use std::collections::HashMap;

use httpmock::prelude::*;
use pocketbase_sdk::client::Client;
use serde_json::json;

const EMAIL: &str = "sreedev@icloud.com";
const PASSWORD: &str = "Admin@123";
const USERNAME: &str = "sreedev";

fn get_body() -> HashMap<String, String> {
    HashMap::from([
        (String::from("email"), EMAIL.to_string()),
        (String::from("password"), PASSWORD.to_string()),
        (String::from("passwordConfirm"), PASSWORD.to_string()),
        (String::from("username"), USERNAME.to_string()),
    ])
}

#[tokio::test]
async fn create_user() {
    let mockserver = mock_user_register();
    let mut client = Client::new(mockserver.url("/api/").as_str()).unwrap();
    let auth = client.register_user(EMAIL, PASSWORD, Some(USERNAME)).await;

    assert!(auth.is_ok(), "Auth error: {}", auth.err().unwrap());
}

fn mock_user_register() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/users/records")
            .json_body_obj(&get_body());

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
              "id": "RECORD_ID",
              "collectionId": "_pb_users_auth_",
              "collectionName": "users",
              "username": USERNAME,
              "verified": false,
              "emailVisibility": true,
              "email": EMAIL,
              "created": "2022-01-01 01:00:00.123Z",
              "updated": "2022-01-01 23:59:59.456Z",
              "avatar": "filename.jpg"
            }
            ));
    });

    server
}
