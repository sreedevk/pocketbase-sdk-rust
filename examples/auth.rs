use anyhow::Result;
use pocketbase_sdk::client::Client;

fn main() -> Result<()> {
    env_logger::init();

    let base = "http://localhost:8090";

    let methods = Client::new(base).auth("users").list_methods()?;
    dbg!(&methods);

    let otp = Client::new(base).auth("users").request_otp("user@example.com")?;
    let authed = Client::new(base).auth("users").with_otp(&otp.otp_id, "123456")?;

    let refreshed = authed.auth("users").refresh()?;
    dbg!(refreshed.auth_token.is_some());

    Client::new(base).auth("users").request_password_reset("user@example.com")?;

    Ok(())
}
