use httpmock::prelude::*;
use pocketbase_sdk::admin::Admin;
use serde_json::json;

#[test]
fn collections_list_success() {
    let mockserver_url = mockserver().base_url();
    let admin_client = Admin::new(mockserver_url.as_str())
        .auth_with_password("sreedev@icloud.com", "Sreedev123")
        .unwrap();

    let collections_list = admin_client.collections().list().call();
    assert!(collections_list.is_ok())
}

#[test]
fn colletion_view_succes() {
    let mockserver_url = mockserver().base_url();
    let admin_client = Admin::new(mockserver_url.as_str())
        .auth_with_password("sreedev@icloud.com", "Sreedev123")
        .unwrap();
    let collection = admin_client.collections().view("posts").call();
    assert!(collection.is_ok())
}

fn mockserver() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET)  
        .path("/api/collections/posts")
            .header("Authorization", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6InN5d2JoZWNuaDQ2cmhtMCIsInR5cGUiOiJhZG1pbiIsImV4cCI6MjIwODk4MTYwMH0.han3_sG65zLddpcX2ic78qgy7FKecuPfOpFa8Dvi5Bg");
            then.status(200).header("content-type", "application/json")
            .json_body(json!({
                "id": "d2972397d45614e",
                "created": "2022-06-22 07:13:00.643Z",
                "updated": "2022-06-22 07:13:00.643Z",
                "name": "posts",
                "type": "base",
                "schema": [
                {
                    "system": false,
                    "id": "njnkhxa2",
                    "name": "title",
                    "type": "text",
                    "required": false,
                    "unique": false,
                    "options": {
                    "min": null,
                    "max": null,
                    "pattern": ""
                }
            },
                {
                    "system": false,
                    "id": "9gvv0jkj",
                    "name": "image",
                    "type": "file",
                    "required": false,
                    "unique": false,
                    "options": {
                    "maxSelect": 1,
                    "maxSize": 5242880,
                    "mimeTypes": [
                    "image/jpg",
                    "image/jpeg",
                    "image/png",
                    "image/svg+xml",
                    "image/gif"
                ],
                "thumbs": null
            }
            }
            ],
                "listRule": "id = @request.user.id",
                "viewRule": "id = @request.user.id",
                "createRule": "id = @request.user.id",
                "updateRule": "id = @request.user.id",
                "deleteRule": null,
                "options": {},
                "indexes": ["create index title_idx on posts (title)"]
            }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/collections")
            .header("Authorization", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6InN5d2JoZWNuaDQ2cmhtMCIsInR5cGUiOiJhZG1pbiIsImV4cCI6MjIwODk4MTYwMH0.han3_sG65zLddpcX2ic78qgy7FKecuPfOpFa8Dvi5Bg");

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!(
            {
            "page": 1,
            "perPage": 100,
            "totalItems": 3,
            "items": [
            {
            "id": "d2972397d45614e",
            "created": "2022-06-22 07:13:00.643Z",
            "updated": "2022-06-22 07:13:00.643Z",
            "name": "users",
            "type": "base",
            "system": true,
            "schema": [
                {
                    "system": false,
                    "id": "njnkhxa2",
                    "name": "title",
                    "type": "text",
                    "required": false,
                    "unique": false,
                    "options": {
                        "min": "",
                        "max": "",
                        "pattern": ""
                    }
                },
                {
                    "system": false,
                    "id": "9gvv0jkj",
                    "name": "avatar",
                    "type": "file",
                    "required": false,
                    "unique": false,
                    "options": {
                        "maxSelect": 1,
                        "maxSize": 5242880,
                        "mimeTypes": [
                            "image/jpg",
                            "image/jpeg",
                            "image/png",
                            "image/svg+xml",
                            "image/gif"
                        ],
                        "thumbs": null
                    }
                }
            ],
            "listRule": "id = @request.user.id",
            "viewRule": "id = @request.user.id",
            "createRule": "id = @request.user.id",
            "updateRule": "id = @request.user.id",
            "deleteRule": null,
                "options": {
                    "manageRule": null,
                    "allowOAuth2Auth": true,
                    "allowUsernameAuth": true,
                    "allowEmailAuth": true,
                    "requireEmail": true,
                    "exceptEmailDomains": [],
                    "onlyEmailDomains": [],
                    "minPasswordLength": 8
                },
            "indexes": ["create index title_idx on users (title)"]
        },
        ]
        }
        ));
    });
    server.mock(|when, then| {
        when
            .method(POST)
            .json_body(json!({
                "identity": "sreedev@icloud.com",
                "password": "Sreedev123"
            }))
            .path("/api/admins/auth-with-password");

        then
            .status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6InN5d2JoZWNuaDQ2cmhtMCIsInR5cGUiOiJhZG1pbiIsImV4cCI6MjIwODk4MTYwMH0.han3_sG65zLddpcX2ic78qgy7FKecuPfOpFa8Dvi5Bg",
                    "admin": {
                    "id": "b6e4b08274f34e9",
                    "created": "2022-06-22 07:13:09.735Z",
                    "updated": "2022-06-22 07:13:09.735Z",
                    "email": "test@example.com",
                    "avatar": 0
                }
            }));
    });

    server
}
