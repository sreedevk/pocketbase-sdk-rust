# Pocketbase SDK for Rust Clients

This project is a work in progress. Feel free to contribute by forking & raising PRs.

# Usage
```rust
use pocketbase_sdk::{Client, admins, users};
use pocketbase_sdk::records::Changeset;
use serde::Serialize;

/* example model */
#[derive(Debug, Serialize)]
pub struct Post<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub author: &'a str
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* create a pocketbase client */
    let client = Client::new("http://localhost:8090/api/").unwrap();

    /* user authentication details */
    let userauthresp = users::Auth::via_email(
        String::from("sreedev@icloud.com"), /* email */
        String::from("Admin@123"),          /* password */
        &client
    ).await.unwrap();

    /* retrieve user after authentication */
    let user = users::Auth::get_user(&user_result).await;

    /* data attribute will be set to None if Authentication failed */
    match user.data {
        Some(userdata) => {
            /* new model entry */
            let post = Post {
                title: "Created via Rust",
                content: "using pocketbase_sdk",
                author: userdata.id.as_str()
            };

            /* create a changeset from new model struct */
            let changeset: Changeset<Post> = Changeset {
                user: &user,
                resource: "posts",
                record: &post
            };

            /* insert into database */
            match Changeset::insert(&client, changeset).await {
                Ok(response) => println!("{:#?}", response),
                Err(_) => println!("could not insert record")
            };
        },
        None => {
            println!("user authentication filed")
        }
    }

    Ok(())
}
```
