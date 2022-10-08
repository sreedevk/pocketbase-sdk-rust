use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[test]
fn authenticate_user() {
    let mut client = pocketbase_sdk::Client::new("localhost:8090/api").unwrap();
    client.authenticate_via_email(
        String::from("sreedev@icloud.com"),
        String::from("Admin@123"),
        hashmap!("admin" => "false")
    );
}

