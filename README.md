# Pocketbase SDK for Rust Clients

This project is a work in progress. Feel free to contribute by forking & raising PRs.

# Installation

```bash
  $ cargo add pocketbase-sdk
```
or add the following to your `Cargo.toml`

```toml
[dependencies]
pocketbase-sdk = "0.0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0.145", features = ["derive"] }
```

# Usage
```rust
use pocketbase_sdk::{admins, users};
use pocketbase_sdk::Client;
use pocketbase_sdk::records::Changeset;
use pocketbase_sdk::records::operations;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub author: &'a str
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:8090/api/").unwrap();
    let result = admins::Auth::via_email(
        String::from("sreedevpadmakumar@gmail.com"),
        String::from("Admin@1234"),
        &client
    ).await;
    println!("{:#?}", result);

    let user_result = users::Auth::via_email(
        String::from("sreedev@icloud.com"),
        String::from("Admin@123"),
        &client
    ).await.unwrap();

    let user = users::Auth::get_user(&user_result).await;
    match user.data {
        Some(userdata) => {
            let post = Post {
                title: "Created via Rust",
                content: "using pocketbase_sdk",
                author: userdata.id.as_str()
            };

            let changeset: Changeset<Post> = Changeset {
                user: &user,
                resource: "posts",
                record: &post
            };

            match operations::insert(changeset, &client).await {
                Ok(response) => println!("{:#?}", response),
                Err(_) => println!("could not insert record")
            };
        },
        None => {
            println!("user authentication filed")
        }
    }

    println!("{:#?}", &user);

    Ok(())
}

```
