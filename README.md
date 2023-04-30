### Pocketbase SDK

A Rust SDK for Pocketbase Clients. Pocketbase is an open source backend for your SaaS & Mobile Applications. The Goal of this project is to create a wrapper around the APIs that Pocketbase exposes to abstract away unnecessary details of implementation, so that you can focus on building your app and not worry about integration with pocketbase.  

#### Currently Compatible with Pocketbase Version 0.15.1

# Installation

```bash
  $ cargo add pocketbase-sdk
  $ cargo add serde
```
or add the following to your `Cargo.toml`

```toml
[dependencies]
pocketbase-sdk = "0.0.7"
serde = { version = "1.0.145", features = ["derive"] }
```

# Usage

### Collections

```rust
use anyhow::Result;
use pocketbase_sdk::admin::Admin;

fn main() -> Result<()> {
    env_logger::init();

    // admin authentication
    let authenticated_admin_client = Admin::new("http://localhost:8090")
        .auth_with_password("sreedev@icloud.com", "Sreedev123")?;

    // collections list + Filter
    let collections = authenticated_admin_client
        .collections()
        .list()
        .page(1)
        .per_page(100)
        .call()?;

    dbg!(collections);

    // view collection
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
    let authenticated_client = Client::new("http://localhost:8090").authenticate_with_password(
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

### Logs

```rust
use anyhow::Result;
use pocketbase_sdk::admin::Admin;

fn main() -> Result<()> {
    env_logger::init();

    // admin authentication
    let admin = Admin::new("http://localhost:8090")
        .auth_with_password("sreedev@icloud.com", "Sreedev123")?;

    // list logs
    let logs = admin.logs().list().page(1).per_page(10).call()?;
    dbg!(&logs);

    // view log
    let somelogid = &logs.items[0].id;
    let logitem = admin.logs().view(somelogid).call()?;
    dbg!(logitem);

    // view log statistics data points
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

* [ ] Collections
    * [x] List Collections
    * [x] View Collection
    * [ ] Create Collection
    * [ ] Auth Refresh
    * [ ] Request Password Reset
    * [ ] Confirm Password Reset
    * [ ] List Admins
    * [ ] View Admin
    * [ ] Create Admin
    * [ ] Update Admin
    * [ ] Delete Admin
* [ ] Files
    * [ ] Download / Fetch File
    * [ ] Generate Protected File Token
* [ ] Records
    * [x] Create Records
    * [x] Update Records
    * [x] Delete Records
    * [ ] Bulk Delete Records
    * [ ] List Auth Methods
    * [ ] Auth with OAuth2
    * [ ] Auth Refresh
    * [ ] Request Verification
    * [ ] Confirm Verification
    * [ ] Request Password Reset
    * [ ] Request Email Change
    * [ ] Confirm Email Change
    * [ ] List Linked External Auth Providers
    * [ ] Unlink External Auth Provider
* [ ] Real Time APIs
* [ ] WebAsm Support
* [ ] Settings
    * [ ] List
    * [ ] Update
* [x] Health Check
