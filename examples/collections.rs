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
        .filter("name = 'employees'".to_string())
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
