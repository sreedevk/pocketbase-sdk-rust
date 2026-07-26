use httpmock::prelude::*;
use pocketbase_sdk::client::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Default, Deserialize)]
pub struct Record {
    pub id: String,
    pub title: String,
}

#[test]
fn list_records_success() {
    let mockserver = mock_records_server();
    let client = Client::new(mockserver.base_url().as_str()).auth_with_password(
        "users",
        "sreedev@icloud.com",
        "Sreedev123",
    ).unwrap();

    let records = client.records("posts").list().per_page(1010).call::<Record>();
    assert!(records.is_ok());
}

fn mock_records_server() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/collections/posts/records")
            .query_param("page", "1")
            .query_param("perPage", "1010")
            .header("Authorization", "eyJhbGciOiJIUzI1NiJ9.eyJpZCI6IjRxMXhsY2xtZmxva3UzMyIsInR5cGUiOiJhdXRoUmVjb3JkIiwiY29sbGVjdGlvbklkIjoiX3BiX3VzZXJzX2F1dGhfIiwiZXhwIjoyMjA4OTg1MjYxfQ.UwD8JvkbQtXpymT09d7J6fdA0aP9g4FJ1GPh_ggEkzc");
        then.header("Content-Type", "application/json")
            .json_body(json!({
                "page": 1,
                "perPage": 100,
                "totalItems": 2,
                "items": [
                {
                    "id": "ae40239d2bc4477",
                    "collectionId": "a98f514eb05f454",
                    "collectionName": "posts",
                    "updated": "2022-06-25 11:03:50.052",
                    "created": "2022-06-25 11:03:35.163",
                    "title": "test1"
                },
                {
                "id": "d08dfc4f4d84419",
                "collectionId": "a98f514eb05f454",
                "collectionName": "posts",
                "updated": "2022-06-25 11:03:45.876",
                "created": "2022-06-25 11:03:45.876",
                "title": "test2"
            }
            ]
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
