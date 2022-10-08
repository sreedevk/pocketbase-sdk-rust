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
use pocketbase_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* new client + authentication */
    let client = Client::new("http://localhost:8090/api/").unwrap();
    let auth = client.auth_via_email(
        String::from("sreedev@icloud.com"),
        String::from("Admin@123"),
        UserTypes::User
    ).await;
    assert!(auth.is_ok())

    Ok(())
}

```
