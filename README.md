### Pocketbase SDK

A Rust SDK for Pocketbase Clients. Pocketbase is an open source backend for your SaaS & Mobile Applications. The Goal of this project is to create a wrapper around the APIs that Pocketbase exposes to abstract away unnecessary details of implementation, so that you can focus on building your app and not worry about integration with pocketbase.

> [!NOTE]
> This project is maintained with the assistance of AI tools. All changes are subject to manual review and a comprehensive test suite to ensure stability and quality.

#### Currently Compatible with Pocketbase Version 0.39.x

#### NOTE
Version 0.2.0 targets PocketBase 0.39.x and contains breaking changes from the 0.1.x line: admin auth has been replaced by superusers, collection and log response shapes have been updated to match the current API, and pagination has been fixed.

# Installation

```bash
  $ cargo add pocketbase-sdk
  $ cargo add serde
```
or add the following to your `Cargo.toml`

```toml
[dependencies]
pocketbase-sdk = "0.4.0"
serde = { version = "1", features = ["derive"] }
```

# Usage

```rust
use anyhow::Result;
use pocketbase_sdk::client::Client;

fn main() -> Result<()> {
    env_logger::init();

    let authenticated_admin_client = Client::new("http://localhost:8090")
        .superusers()
        .auth_with_password("sreedev@icloud.com", "Sreedev123")?;

    let collections = authenticated_admin_client
        .collections()
        .list()
        .page(1)
        .per_page(100)
        .call()?;

    dbg!(collections);

    let user_collection = authenticated_admin_client
        .collections()
        .view("users")
        .call()?;

    dbg!(user_collection);

    Ok(())
}
```

### Records
```rust
use anyhow::Result;
use pocketbase_sdk::client::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewProduct {
    pub name: String,
    pub count: i32,
}

fn main() -> Result<()> {
    env_logger::init();

    /* Authenticate Client */
    let authenticated_client = Client::new("http://localhost:8090").auth_with_password(
        "users",
        "sreedev@icloud.com",
        "Sreedev123",
    )?;

    /* List Products */
    let products = authenticated_client
        .records("products")
        .list()
        .call::<Product>()?;
    dbg!(products);

    /* View Product */
    let product = authenticated_client
        .records("products")
        .view("jme4ixxqie2f9ho")
        .call::<Product>()?;
    dbg!(product);

    /* Create Product */
    let new_product = NewProduct {
        name: String::from("bingo"),
        count: 69420,
    };
    let create_response = authenticated_client
        .records("products")
        .create(new_product)
        .call()?;
    dbg!(&create_response);

    /* Update Product */
    let updated_product = NewProduct {
        name: String::from("bango"),
        count: 69420,
    };
    let update_response = authenticated_client
        .records("products")
        .update(create_response.id.as_str(), updated_product)
        .call()?;

    dbg!(update_response);

    /* Delete Product */
    authenticated_client
        .records("products")
        .destroy(create_response.id.as_str())
        .call()?;

    Ok(())
}
```

### File Upload & Query Params

```rust
let products = authenticated_client
    .records("products")
    .list()
    .expand("author")
    .fields("id,title")
    .call::<Product>()?;

let create_response = authenticated_client
    .records("products")
    .create_multipart()
    .text("title", "hello")
    .file_bytes("photo", "photo.png", photo_bytes)
    .call()?;
```

### Batch

```rust
let results = client
    .batch()
    .create("posts", &json!({ "title": "one" }))?
    .delete("posts", "someoldid")
    .call()?;
```

### Files

```rust
let token = client.files().token()?;

let bytes = client
    .files()
    .download("posts", "recordid", "photo.png")
    .thumb("100x100")
    .token(&token)
    .call()?;
```

### Auth Flows

```rust
use pocketbase_sdk::client::Client;

let methods = Client::new("http://localhost:8090").auth("users").list_methods()?;

let otp = Client::new("http://localhost:8090").auth("users").request_otp("user@example.com")?;
let client = Client::new("http://localhost:8090").auth("users").with_otp(&otp.otp_id, "123456")?;

Client::new("http://localhost:8090").auth("users").request_password_reset("user@example.com")?;

let refreshed = client.auth("users").refresh()?;
```

### Logs

```rust
use anyhow::Result;
use pocketbase_sdk::client::Client;

fn main() -> Result<()> {
    env_logger::init();

    let admin = Client::new("http://localhost:8090")
        .superusers()
        .auth_with_password("sreedev@icloud.com", "Sreedev123")?;

    let logs = admin.logs().list().page(1).per_page(10).call()?;
    dbg!(&logs);

    let somelogid = &logs.items[0].id;
    let logitem = admin.logs().view(somelogid).call()?;
    dbg!(logitem);

    let logstats = admin.logs().statistics().call()?;
    dbg!(logstats);

    Ok(())
}
```

### HealthCheck

```rust
use anyhow::Result;
use pocketbase_sdk::client::Client;

fn main() -> Result<()> {
    let client = Client::new("http://localhost:8090");
    let health_check_response = client.health_check()?;
    dbg!(health_check_response);

    Ok(())
}
```

# Development TODOs
* [ ] Improve Test Coverage
* [x] Superuser Auth
* [ ] Collections
    * [x] List Collections
    * [x] View Collection
    * [ ] Create Collection
* [ ] Logs
    * [x] List Logs
    * [x] View Log
    * [x] Log Statistics
* [ ] Files
    * [x] Download / Fetch File
    * [x] Generate Protected File Token
* [ ] Records
    * [x] Create Records
    * [x] Update Records
    * [x] Delete Records
    * [x] Bulk Delete Records
    * [x] List Auth Methods
    * [x] Auth with OAuth2
    * [x] Auth Refresh
    * [x] Request Verification
    * [x] Confirm Verification
    * [x] Request Password Reset
    * [x] Request Email Change
    * [x] Confirm Email Change
    * [ ] List Linked External Auth Providers
    * [ ] Unlink External Auth Provider
* [ ] Real Time APIs
* [ ] WebAsm Support
* [ ] Settings
    * [ ] List
    * [ ] Update
* [x] Health Check
