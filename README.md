<h3 align="center">Pocketbase SDK</h3>

This project is a work in progress. Feel free to contribute by forking & raising PRs.

# Installation

```bash
  $ cargo add pocketbase-sdk
```
or add the following to your `Cargo.toml`

```toml
[dependencies]
pocketbase-sdk = "0.0.5"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0.145", features = ["derive"] }
```

# Usage
```rust
use pocketbase_sdk::client::Client;
use pocketbase_sdk::records::operations::{create, delete, list, view};
use pocketbase_sdk::user::UserTypes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Default)]
struct Post {
    id: String,
    title: String,
    content: String,
    created: String,
    updated: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* new client + authentication */
    let mut client = Client::new("http://localhost:8090/api/").unwrap();
    let auth = client
        .auth_via_email(
            "sreedev@icloud.com",
            "Admin@123",
            UserTypes::User, /* use UserTypes::Admin for admin Authentication */
        )
        .await;
    assert!(auth.is_ok());

    /* create record */
    let record = Post {
        title: "Sample title".to_string(),
        content: "Sample Content".to_string(),
        author: client.user.as_ref().unwrap().token.clone(),,
        ..Default::default()
    };

    let response = create::record::<Post>("posts", &record, &client)
        .await
        .unwrap();
    let id = match response {
        create::CreateResponse::SuccessResponse(res) => {
            assert_eq!(res.title, String::from("Sample title"));
            res.id.clone()
        }
        create::CreateResponse::FailureResponse(_err) => panic!("Failed!"),
    };

    /* view record */
    let response = view::record::<Post>("posts", &id, &client).await.unwrap();
    match response {
        view::ViewResponse::SuccessResponse(res) => assert_eq!(&res.id, &id),
        view::ViewResponse::ErrorResponse(_err) => panic!("Failed!"),
    }

    /* list paginated records */
    let response = list::records::<Post>("posts", &client, None).await.unwrap();
    match response {
        list::ListResponse::SuccessResponse(paginated_record_list) => {
            assert_ne!(paginated_record_list.total_items, 0)
        }
        list::ListResponse::ErrorResponse(_e) => panic!("could not retrieve resource."),
    }

    /* delete a record */
    let response = delete::record("posts", &id, &client).await;
    assert!(response.is_ok());

    Ok(())
}

```

# Roadmap

1. Add File Upload Options
2. Add Log Interface
3. Add Admin Settings Interface
4. Realtime API Options
5. WebAsm Support (v0.0.6)
