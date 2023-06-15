#[macro_use]
extern crate rocket;

pub mod auth;
pub mod comment;
pub mod recipe;
pub mod schema;
pub mod user;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

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
        .mount("/api/user", user::routes::index())
        .mount("/api/recipe", recipe::routes::index())
        .mount("/api/comment", comment::routes::index())
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
