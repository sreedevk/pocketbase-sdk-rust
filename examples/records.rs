use anyhow::Result;
use pocketbase_sdk::client::{Client, Credentials};
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
    /* Authenticate Client */
    let client               = Client::new("http://localhost:8090");
    let credentials          = Credentials::new("users", "sreedev@icloud.com", "Sreedev123");
    let authenticated_client = client.authenticate_with_password(credentials)?;

    /* List Products */
    let products = authenticated_client.records("products").list().call::<Product>()?;
    dbg!(products);

    /* View Product */
    let product = authenticated_client.records("products").view("jme4ixxqie2f9ho").call::<Product>()?;
    dbg!(product);

    /* Create Product */
    let new_product = NewProduct {
        name: String::from("bingo"),
        count: 69420
    };
    let create_response = authenticated_client.records("products").create(new_product).call()?;
    dbg!(create_response);

    Ok(())
}
