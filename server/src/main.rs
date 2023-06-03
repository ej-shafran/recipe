#[macro_use]
extern crate rocket;

pub mod schema;
pub mod user;

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
        .mount("/api", routes![login, register])
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
pub async fn login(user: Json<UserDTO>, mut db: Connection<DB>) -> Result<String, Status> {
    user::login(&user.into_inner(), &mut *db).await
}

#[post("/register", data = "<user>")]
pub async fn register(user: Json<UserDTO>, mut db: Connection<DB>) -> Result<String, Status> {
    user::register(&user.into_inner(), &mut db).await
}
