use pocketbase_sdk::client::SyncClient;
use pocketbase_sdk::records::sync_delete;
use httpmock::prelude::*;

#[test]
fn delete_record() {
    let server = mock_list_view();
    let client = SyncClient::new(server.url("/api/").as_str()).unwrap();
    let response = sync_delete::record("posts", "9bbl183t7ioqrea", &client);
    assert!(response.is_ok());
}

fn mock_list_view() -> MockServer {
    let server = MockServer::start();

    server.mock(|when, then| {
        when
            .method(DELETE)
            .path("/api/collections/posts/records/9bbl183t7ioqrea");

        then
            .status(204)
            .header("content-type", "application/json");
    });

    server
}
