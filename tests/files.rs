use httpmock::prelude::*;
use pocketbase_sdk::client::Client;
use serde_json::json;

#[test]
fn file_token_returns_token() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/api/collections/users/auth-with-password");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": "authtok", "record": { "id": "u" } }));
    });
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/files/token")
            .header("Authorization", "authtok");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "token": "filetok" }));
    });

    let client = Client::new(server.base_url().as_str())
        .auth_with_password("users", "i", "p")
        .unwrap();
    let token = client.files().token();
    assert_eq!(token.unwrap(), "filetok");
}

#[test]
fn file_download_returns_bytes() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/files/posts/rec1/photo.png")
            .query_param("thumb", "100x100")
            .query_param("token", "filetok")
            .query_param("download", "true");
        then.status(200).body("rawimagebytes");
    });

    let client = Client::new(server.base_url().as_str());
    let bytes = client
        .files()
        .download("posts", "rec1", "photo.png")
        .thumb("100x100")
        .token("filetok")
        .download(true)
        .call();
    assert_eq!(bytes.unwrap(), b"rawimagebytes".to_vec());
}

#[test]
fn file_url_builds_path() {
    let client = Client::new("http://localhost:8090");
    let url = client.files().url("posts", "rec1", "photo.png");
    assert_eq!(url, "http://localhost:8090/api/files/posts/rec1/photo.png");
}
