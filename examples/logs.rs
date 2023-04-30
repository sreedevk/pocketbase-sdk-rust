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
