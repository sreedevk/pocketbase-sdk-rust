### Pocketbase SDK

A Rust SDK for Pocketbase Clients. Pocketbase is an open source backend for your SaaS & Mobile Applications. The Goal of this project is to create a wrapper around the APIs that Pocketbase exposes to abstract away unnecessary details of implementation, so that you can focus on building your app and not worry about integration with pocketbase.  

#### Currently Compatible with Pocketbase Version 0.15.1

# Installation

```bash
  $ cargo add pocketbase-sdk
```
or add the following to your `Cargo.toml`

```toml
[dependencies]
pocketbase-sdk = "0.0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0.145", features = ["derive"] }
```

# Usage
```rust
use pocketbase_sdk::client::{Client, Credentials};
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    let pocket_client        = Client::new("http://localhost:8090");
    let auth_info            = Credentials::new("users", "sreedev@icloud.com", "Sreedev123");
    let authenticated_client = pocket_client.authenticate_with_password(auth_info)?;
    let collections          = authenticated_client.collections().list()?;

    dbg!(&authenticated_client);
    dbg!(collections);

    Ok(())
}
```
