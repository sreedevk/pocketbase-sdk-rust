use httpmock::Method::PATCH;
use pocketbase_sdk::client::SyncClient;
use pocketbase_sdk::records::sync_update;
use httpmock::prelude::*;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: String,
    title: String,
    content: String,
    created: String,
    updated: String,
    author: String,
}

#[test]
fn update_record() {
    let server = mock_record_create();
    let client = SyncClient::new(server.url("/api/").as_str()).unwrap();
    let post = Post {
        id: "".to_string(),
        title: "Test Post Created By Pocketbase SDK".to_string(),
        content: "This is a test post".to_string(),
        created: "".to_string(),
        updated: "".to_string(),
        author: "jxso1raa3ta3p0y".to_string()
    };
    let repsonse = sync_update::record::<Post>("posts", "jxso1raa3ta3p0y", &post, &client).unwrap();
    match repsonse {
        sync_update::CreateResponse::SuccessResponse(res) => {
            assert_eq!(res.title, String::from("Test Post Created By Pocketbase SDK"))
        },
        sync_update::CreateResponse::FailureResponse(_err) => panic!("Failed!")
    }
}

fn mock_record_create() -> MockServer {
    let server = MockServer::start();

    server.mock(|when, then| {
        when
            .method(PATCH)
            .path("/api/collections/posts/records/jxso1raa3ta3p0y");

        then
            .status(200)
            .header("content-type", "application/json")
            .json_body(
                json!(
                    {
                        "@collectionId": "ba47n093oe2awj7",
                        "@collectionName": "posts",
                        "author": "jxso1raa3ta3p0y",
                        "content": "This is a test post",
                        "created": "2022-10-05 11:21:11.444",
                        "id": "9bbl183t7ioqrea",
                        "title": "Test Post Created By Pocketbase SDK",
                        "updated": "2022-10-05 11:21:11.444"
                    }
                )
            );
    });

    server
}
