use pocketbase_sdk::client::Client;
use pocketbase_sdk::records::operations::{create, delete, list, view};
use pocketbase_sdk::user::UserTypes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Default)]
struct Post {
    id: String,
    title: String,
    content: String,
    created: String,
    updated: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* new client + authentication */
    let mut client = Client::new("http://localhost:8090/api/").unwrap();
    let auth = client
        .auth_via_email(
            String::from("sreedev@icloud.com"),
            String::from("Admin@123"),
            UserTypes::User, /* use UserTypes::Admin for admin Authentication */
        )
        .await;
    assert!(auth.is_ok());

    /* create record */
    let record = Post {
        id: "".to_string(),
        title: "Sample title".to_string(),
        content: "Sample Content".to_string(),
        author: client.user.as_ref().unwrap().token.clone(),
        created: "".to_string(),
        updated: "".to_string(),
    };

    let response = create::record::<Post>("posts", &record, &client)
        .await
        .unwrap();
    let id = match response {
        create::CreateResponse::SuccessResponse(res) => {
            assert_eq!(res.title, String::from("Sample title"));
            res.id.clone()
        }
        create::CreateResponse::FailureResponse(_err) => panic!("Failed!"),
    };

    /* view record */
    let response = view::record::<Post>("posts", &id, &client).await.unwrap();
    match response {
        view::ViewResponse::SuccessResponse(res) => assert_eq!(&res.id, &id),
        view::ViewResponse::ErrorResponse(_err) => panic!("Failed!"),
    }

    /* list paginated records */
    let response = list::records::<Post>("posts", &client, None).await.unwrap();
    match response {
        list::ListResponse::SuccessResponse(paginated_record_list) => {
            assert_ne!(paginated_record_list.total_items, 0)
        }
        list::ListResponse::ErrorResponse(_e) => panic!("could not retrieve resource."),
    }

    /* delete a record */
    let response = delete::record("posts", &id, &client).await;
    assert!(response.is_ok());

    Ok(())
}
