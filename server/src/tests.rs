use crate::user::UserDTO;
use rocket::{http::Status, local::asynchronous::Client, serde::json};
use std::{process::Command, sync::Once};

fn test_user() -> UserDTO {
    UserDTO {
        username: String::from("TEST_USER"),
        password: String::from("12345678"),
    }
}

fn incorrect_user() -> UserDTO {
    UserDTO {
        username: String::from("INCORRECT"),
        password: String::from("INCORRECT"),
    }
}

fn new_user() -> UserDTO {
    UserDTO {
        username: String::from("NEW_USER"),
        password: String::from("abcdefgh"),
    }
}

static INIT: Once = Once::new();

async fn initialize() -> Client {
    INIT.call_once(|| {
        Command::new("sqlx")
            .args(["database", "reset"])
            .output()
            .expect("failed to reset database");
    });

    Client::tracked(super::rocket()).await.unwrap()
}

#[async_test]
async fn login() {
    let client = initialize().await;

    let req = client
        .post("/api/user/login")
        .body(json::to_string(&test_user()).unwrap())
        .dispatch()
        .await;
    assert_eq!(
        req.status(),
        Status::Ok,
        "successfully login with existing credentials"
    );

    let req = client
        .post("/api/user/login")
        .body(json::to_string(&incorrect_user()).unwrap())
        .dispatch()
        .await;
    assert_eq!(
        req.status(),
        Status::Unauthorized,
        "login with nonexistant credentials"
    );
}

#[async_test]
async fn register() {
    let client = initialize().await;

    let req = client
        .post("/api/user/register")
        .body(json::to_string(&new_user()).unwrap())
        .dispatch()
        .await;
    assert_eq!(req.status(), Status::Ok, "initial register");

    let req = client
        .post("/api/user/login")
        .body(json::to_string(&new_user()).unwrap())
        .dispatch()
        .await;
    assert_eq!(req.status(), Status::Ok, "login with created user");

    let req = client
        .post("/api/user/register")
        .body(json::to_string(&test_user()).unwrap())
        .dispatch()
        .await;
    assert_eq!(req.status(), Status::Unauthorized, "register existing user");
}
