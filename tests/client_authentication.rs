use pocketbase_sdk::Client;
use pocketbase_sdk::UserTypes;
use httpmock::prelude::*;

#[tokio::test]
async fn authenticate_user() {
    let mockserver = mock_pocketbase_api();
    let mut client = Client::new(mockserver.url("/api").as_str()).unwrap();
    let auth = client.auth_via_email(
        String::from("sreedev@icloud.com"),
        String::from("Admin@123"),
        UserTypes::User
    ).await;

    assert!(auth.is_ok());
}

fn mock_pocketbase_api() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when
            .method(POST)
            .path("/api/users/auth-via-email")
            .query_param("type", "success");

        then
            .status(200)
            .body(
                r#"
                    {
                        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NjY0Njg3ODQsImlkIjoianhzbzFyYWEzdGEzcDB5IiwidHlwZSI6InVzZXIifQ.FIeDD2RE7Gz4WOe8eVIEC765l0llXA9d-wUumXBLYm8",
                        "user": {
                            "id": "jxso1raa3ta3p0y",
                            "created": "2022-10-05 11:19:32.545",
                            "updated": "2022-10-05 11:19:32.545",
                            "email": "adminuser@gmail.com",
                            "lastResetSentAt": "",
                            "verified": false,
                            "lastVerificationSentAt": "",
                            "profile": {
                                "@collectionId": "systemprofiles0",
                                "@collectionName": "profiles",
                                "avatar": "",
                                "created": "2022-10-05 11:19:32.545",
                                "id": "bpnv2fx3e098x6w",
                                "name": "",
                                "updated": "2022-10-05 11:19:32.545",
                                "userId": "jxso1raa3ta3p0y"
                            }
                        }
                    }
                "#
            );
    });

    server
}
