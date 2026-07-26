use httpmock::prelude::*;
use pocketbase_sdk::client::Client;
use serde_json::json;

#[test]
fn logs_list_success() {
    let server = mock_logs_server();
    let client = Client::new(server.base_url().as_str())
        .superusers()
        .auth_with_password("sreedev@icloud.com", "Sreedev123")
        .unwrap();

    let logs = client.logs().list().call();
    assert!(logs.is_ok());
    assert_eq!(logs.unwrap().items[0].level, 0);
}

fn mock_logs_server() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/logs");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "page": 1,
                "perPage": 100,
                "totalItems": 1,
                "items": [
                    {
                        "id": "ai5z3aoh6cs4mv7",
                        "created": "2022-06-22 07:13:00.643Z",
                        "updated": "2022-06-22 07:13:00.643Z",
                        "level": 0,
                        "message": "GET /api/health",
                        "data": {
                            "method": "GET",
                            "status": 200,
                            "url": "/api/health"
                        }
                    }
                ]
            }));
    });
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/collections/_superusers/auth-with-password");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.token",
                "record": { "id": "b6e4b08274f34e9", "email": "test@example.com" }
            }));
    });
    server
}
