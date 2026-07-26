use anyhow::Result;
use pocketbase_sdk::client::Client;
use serde_json::json;

fn main() -> Result<()> {
    env_logger::init();

    let client = Client::new("http://localhost:8090")
        .auth_with_password("users", "sreedev@icloud.com", "Sreedev123")?;

    let results = client
        .batch()
        .create("posts", &json!({ "title": "one" }))?
        .create("posts", &json!({ "title": "two" }))?
        .delete("posts", "someoldid")
        .call()?;

    dbg!(results);
    Ok(())
}
