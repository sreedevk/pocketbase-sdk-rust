use pocketbase_sdk::client::SyncClient;
use pocketbase_sdk::records::sync_view;
use httpmock::prelude::*;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: String,
    title: String,
    content: String
}

#[test]
fn view_record() {
    let server = mock_list_view();
    let client = SyncClient::new(server.url("/api/").as_str()).unwrap();
    let repsonse = sync_view::record::<Post>("posts", "9bbl183t7ioqrea", &client).unwrap();
    match repsonse {
        sync_view::ViewResponse::SuccessResponse(res) => assert_eq!(res.id, "9bbl183t7ioqrea"),
        sync_view::ViewResponse::ErrorResponse(_err) => panic!("Failed!")
    }
}

fn mock_list_view() -> MockServer {
    let server = MockServer::start();

    server.mock(|when, then| {
        when
            .method(GET)
            .path("/api/collections/posts/records/9bbl183t7ioqrea");

        then
            .status(200)
            .header("content-type", "application/json")
            .json_body(
                json!(
                    {
                        "@collectionId": "ba47n093oe2awj7",
                        "@collectionName": "posts",
                        "author": "jxso1raa3ta3p0y",
                        "content": "User 2Lorem Ipsum Doler",
                        "created": "2022-10-05 11:21:11.444",
                        "id": "9bbl183t7ioqrea",
                        "title": "User 2 Hello World!",
                        "updated": "2022-10-05 11:21:11.444"
                    }
                )
            );
    });

    server
}
