use httpmock::Method::PATCH;
use pocketbase_sdk::{client::Client, records::update::UpdateResponse};
use pocketbase_sdk::records::update;
use httpmock::prelude::*;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Post {
    title: String,
    content: String,
}

#[tokio::test]
async fn update_record() {
    let server = mock_record_patch();
    let client = Client::new(server.url("/api/").as_str()).unwrap();
    let post = Post {
        title: "Test Post Created By Pocketbase SDK".to_string(),
        content: "This is a test post".to_string(),
    };

    let response = update::record::<Post>(
        "posts",
        "jxso1raa3ta3p0y",
        &post,
        &client
    ).await;

    match response {
        Ok(resp) => {
            match resp {
                UpdateResponse::SuccessResponse(mod_post) => {
                    assert_eq!(mod_post.title, post.title)
                },
                UpdateResponse::FailureResponse(_) => panic!("failed")
            }
        },
        Err(e) => println!("{:#?}", e)
    }
    
}

fn mock_record_patch() -> MockServer {
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
