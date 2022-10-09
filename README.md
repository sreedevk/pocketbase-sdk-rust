<h3 align="center">Pocketbase SDK</h3>

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
use pocketbase_sdk::client::Client;
use pocketbase_sdk::user::UserTypes;
use pocketbase_sdk::records::operations::list;

#[derive(PocketbaseModel)]
struct Post {
  title: String,
  content: String,
  published_at: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* new client + authentication */
    let mut client = Client::new("http://localhost:8090/api/").unwrap();
    let auth = client.auth_via_email(
        String::from("sreedev@icloud.com"),
        String::from("Admin@123"),
        UserTypes::User /* use UserTypes::Admin for admin Authentication */
    ).await;
    assert!(auth.is_ok())

    let response = list::records::<Post>("posts", &client).await.unwrap();
    match response {
      ListResponse::SuccessResponse(paginated_record_list) => {
        assert_ne!(paginated_record_list.total_items, 0)
      },
      ListResponse::ErrorResponse(_e) => panic!("could not retrieve resource.")
    }

    Ok(())
}

```
