use anyhow::Result;
use pocketbase_sdk::client::{Client, Credentials};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub count: i32,
}

fn main() -> Result<()> {
    let client               = Client::new("http://localhost:8090");
    let credentials          = Credentials::new("users", "sreedev@icloud.com", "Sreedev123");
    let authenticated_client = client.authenticate_with_password(credentials)?;

    let products = authenticated_client.records("products").list().call::<Product>()?;
    let product = authenticated_client.records("products").view("jme4ixxqie2f9ho").call::<Product>()?;
    dbg!(products);
    dbg!(product);

    Ok(())
}
