use std::{collections::HashMap, iter::Successors};
use httpmock::prelude::*;
use pocketbase_sdk::records::{Recordable, operations::list::*};
use pocketbase_sdk::records::operations::list;
use pocketbase_sdk::client::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: Option<String>,
    body: Option<String>
}

impl Recordable for Post {
    fn new(_args: HashMap<String, String>) -> Post {
        Post { title: None, body: None }
    }
}

#[tokio::test]
async fn list_records() {
    let server = mock_list_posts();
    let client = Client::new(server.url("/api/").as_str()).unwrap();
    let repsonse = list::records::<Post>("posts", &client).await.unwrap();
    match repsonse {
        ListResponse::SuccessResponse(res) => assert_eq!(res.total_items, 1),
        ListResponse::ErrorResponse(_err) => panic!("Failed!")
    }
}

pub fn mock_list_posts() -> MockServer {
    let server = MockServer::start();

    server.mock(|when, then| {
        when
            .method(GET)
            .path("/api/collections/posts/records");

        then
            .status(200)
            .header("content-type", "application/json")
            .json_body(
                json!({
                    "page": 1,
                    "perPage": 30,
                    "totalItems": 1,
                    "totalPages": 1,
                    "items": [
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
                ]
                })
            );
    });

    server
}
