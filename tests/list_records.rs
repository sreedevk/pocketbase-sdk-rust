use std::collections::HashMap;
use httpmock::prelude::*;
use pocketbase_sdk::records::Recordable;
use pocketbase_sdk::records::operations::list;
use pocketbase_sdk::client::Client;

use serde::{Serialize, Deserialize};

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
    //let server = mock_list_posts();
    //let client = Client::new(server.url("/api/").as_str()).unwrap();
    //let posts_list = list::records::<Post>("posts", &client).await.unwrap();
    //println!("{:#?}", posts_list);
    //assert_eq!(posts_list, 1);
}

pub fn mock_list_posts() -> MockServer {
    let server = MockServer::start();

    server.mock(|when, then| {
        when
            .method(POST)
            .path("/api/collections/posts/");

        then
            .status(200)
            .header("content-type", "application/json")
            .body(
                r#"
                    {
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
                    }
                "#
            );
    });

    server
}
