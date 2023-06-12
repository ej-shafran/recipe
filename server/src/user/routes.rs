use crate::{auth, DB};
use rocket::http::CookieJar;
use rocket::Route;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::Connection;

pub fn index() -> Vec<Route> {
    routes![post_login, post_register]
}

#[post("/login", data = "<user>")]
pub async fn post_login(
    user: Json<super::UserDTO>,
    mut db: Connection<DB>,
    cookies: &CookieJar<'_>,
) -> Result<(), Status> {
    let id = super::login(&user.into_inner(), &mut db).await?;

    auth::UserID::add_to_cookie(&id, cookies);

    Ok(())
}

#[post("/register", data = "<user>")]
pub async fn post_register(
    user: Json<super::UserDTO>,
    mut db: Connection<DB>,
    cookies: &CookieJar<'_>,
) -> Result<(), Status> {
    let id = super::register(&user.into_inner(), &mut db).await?;

    auth::UserID::add_to_cookie(&id, cookies);

    Ok(())
}
