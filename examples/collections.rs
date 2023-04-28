use pocketbase_sdk::admin::Admin;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    let authenticated_admin_client = Admin::new("http://localhost:8090", "sreedev@icloud.com", "Sreedev123")?;
    let collections     = authenticated_admin_client.collections().list().page(1).per_page(100).call()?;
    let user_collection = authenticated_admin_client.collections().view("users").call()?;

    dbg!(user_collection);
    dbg!(collections);

    Ok(())
}
