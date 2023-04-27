use pocketbase_sdk::client::{Client, Credentials};
use pocketbase_sdk::admin::Admin;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    let pocket_client           = Client::new("http://localhost:8090");
    let auth_info               = Credentials::new("users", "sreedev@icloud.com", "Sreedev123");
    let authenticated_client    = pocket_client.authenticate_with_password(auth_info)?;

    let authenticated_admin_client = Admin::new("http://localhost:8090", "sreedev@icloud.com", "Sreedev123")?;


    /* Listing Collections */
    let collections = authenticated_admin_client.collections().list().page(1).per_page(100).call()?;

    dbg!(&authenticated_client);
    dbg!(collections);

    Ok(())
}
