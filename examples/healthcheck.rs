use anyhow::Result;
use pocketbase_sdk::client::Client;

fn main() -> Result<()> {
    let client = Client::new("http://localhost:8090");
    let health_check_response = client.health_check()?;
    dbg!(health_check_response);

    Ok(())
}
