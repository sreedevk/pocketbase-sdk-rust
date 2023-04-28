use pocketbase_sdk::admin::Admin;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    let admin = Admin::new("http://localhost:8090", "sreedev@icloud.com", "Sreedev123")?;
    let logs  = admin.logs().list().page(1).per_page(10).call()?;
    let somelogid = &logs.items[0].id.clone();
    let logitem = admin.logs().view(somelogid).call()?;
    let logstats = admin.logs().statistics().call()?;

    dbg!(logs);
    dbg!(logstats);
    dbg!(logitem);
    Ok(())
}
