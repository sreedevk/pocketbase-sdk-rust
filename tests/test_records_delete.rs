use pocketbase_sdk::client::Client;
use pocketbase_sdk::records::delete;
use httpmock::prelude::*;

#[tokio::test]
async fn delete_record() {
    let server = mock_list_view();
    let client = Client::new(server.url("/api/").as_str()).unwrap();
    let response = delete::record("posts", "9bbl183t7ioqrea", &client).await;
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
