use httpmock::prelude::*;
use pocketbase_sdk::client::Client;
use serde_json::json;

#[test]
fn batch_executes_and_parses_results() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/api/collections/users/auth-with-password");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": "tok", "record": { "id": "u" } }));
    });
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/batch")
            .body_contains("requests")
            .body_contains("/api/collections/posts/records")
            .body_contains("/api/collections/posts/records/id1");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!([
                { "status": 200, "body": { "id": "a" } },
                { "status": 200, "body": { "id": "b" } },
                { "status": 204, "body": {} }
            ]));
    });

    let client = Client::new(server.base_url().as_str())
        .auth_with_password("users", "i", "p")
        .unwrap();
    let results = client
        .batch()
        .create("posts", &json!({ "title": "x" }))
        .unwrap()
        .update("posts", "id1", &json!({ "title": "y" }))
        .unwrap()
        .delete("posts", "del1")
        .call();
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].status, 200);
    assert_eq!(results[0].body["id"], "a");
}
