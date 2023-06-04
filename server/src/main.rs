#[macro_use]
extern crate rocket;

pub mod auth;
pub mod schema;
pub mod user;

use auth::UserID;
use rocket::http::CookieJar;
use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket::{Build, Rocket};
use rocket_db_pools::{Connection, Database};
use user::UserDTO;

#[derive(Database)]
#[database("recipe_db")]
pub struct DB(sqlx::MySqlPool);

#[launch]
fn launch() -> _ {
    rocket()
}

pub fn rocket() -> Rocket<Build> {
    dotenvy::dotenv().expect("cannot find dotenv file");

    rocket::build()
        .mount("/api", routes![login, register, testing])
        .attach(DB::init())
        .attach(AdHoc::try_on_ignite("Run migrations", |rocket| async {
            if let Some(db) = DB::fetch(&rocket) {
                if let Err(err) = sqlx::migrate!().run(&db.0).await {
                    dbg!(err);
                    return Err(rocket);
                }
                Ok(rocket)
            } else {
                dbg!("Failed to fetch DB from rocket");
                Err(rocket)
            }
        }))
}

#[post("/login", data = "<user>")]
pub async fn login(
    user: Json<UserDTO>,
    mut db: Connection<DB>,
    cookies: &CookieJar<'_>,
) -> Result<(), Status> {
    let id = user::login(&user.into_inner(), &mut db).await?;

    UserID::add_to_cookie(&id, cookies);

    Ok(())
}

#[post("/register", data = "<user>")]
pub async fn register(
    user: Json<UserDTO>,
    mut db: Connection<DB>,
    cookies: &CookieJar<'_>,
) -> Result<(), Status> {
    let id = user::register(&user.into_inner(), &mut db).await?;

    UserID::add_to_cookie(&id, cookies);

    Ok(())
}

#[get("/testing")]
pub async fn testing(user_id: UserID) -> &'static str {
    dbg!(user_id);
    "Hello, user"
}
