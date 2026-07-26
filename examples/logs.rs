use anyhow::Result;
use pocketbase_sdk::client::Client;

fn main() -> Result<()> {
    env_logger::init();

    let admin = Client::new("http://localhost:8090")
        .superusers()
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
