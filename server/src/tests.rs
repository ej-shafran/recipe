use crate::{recipe::RecipeDTO, user::UserDTO};
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

fn test_recipe() -> RecipeDTO {
    RecipeDTO {
        title: String::from("TEST_RECIPE"),
        content: String::from("TEST_CONTENT"),
    }
}

static INIT: Once = Once::new();

async fn initialize() -> Client {
    INIT.call_once(|| {
        Command::new("sqlx")
            .args(["database", "reset", "-y"])
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

#[async_test]
async fn post_new_recipe() {
    let client = initialize().await;

    let response = client
        .post("/api/user/login")
        .body(json::to_string(&test_user()).unwrap())
        .dispatch()
        .await;

    let auth_cookie_name = std::env::var("AUTH_COOKIE").unwrap();
    let cookie = response.cookies().get(&auth_cookie_name).unwrap();

    //TODO: send the cookie so this test succeeds
    let req = client
        .post("/api/recipe")
        .body(json::to_string(&test_recipe()).unwrap())
        .header(cookie)
        .dispatch()
        .await;
    assert_eq!(req.status(), Status::Ok);
    let body = req.into_string().await.unwrap();
    dbg!(&body);
    assert!(json::from_str::<u64>(&body).is_ok())
}
