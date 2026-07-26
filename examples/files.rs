use anyhow::Result;
use pocketbase_sdk::client::Client;

fn main() -> Result<()> {
    env_logger::init();

    let client = Client::new("http://localhost:8090")
        .auth_with_password("users", "sreedev@icloud.com", "Sreedev123")?;

    let token = client.files().token()?;
    let bytes = client
        .files()
        .download("posts", "recordid", "photo.png")
        .thumb("100x100")
        .token(&token)
        .call()?;

    dbg!(bytes.len());
    Ok(())
}
