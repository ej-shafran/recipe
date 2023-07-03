#[macro_use]
extern crate rocket;

pub mod auth;
pub mod comment;
pub mod recipe;
pub mod schema;
pub mod user;

#[cfg(test)]
mod tests;

use rocket::{Build, Rocket};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("recipe_db")]
pub struct DB(sqlx::MySqlPool);

#[launch]
fn launch() -> _ {
    rocket()
}

#[get("/")]
fn health() {}

pub fn rocket() -> Rocket<Build> {
    let _ = dotenvy::from_filename("../../.env");
    let _ = dotenvy::dotenv();

    rocket::build()
        .mount("/", routes![health])
        .mount("/api/user", user::routes::index())
        .mount("/api/recipe", recipe::routes::index())
        .mount("/api/comment", comment::routes::index())
        .attach(DB::init())
}
