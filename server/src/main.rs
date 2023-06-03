#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

pub mod schema;
pub mod user;

#[derive(Database)]
#[database("recipe_db")]
pub struct DB(sqlx::MySqlPool);

#[launch]
pub fn rocket() -> _ {
    dotenvy::dotenv().expect("cannot find dotenv file");

    rocket::build()
        .mount("/api", routes![user::login, user::register])
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

#[cfg(test)]
mod tests {
    // use super::rocket;
    // use rocket::{http::Status, local::blocking::Client};
    //
    // #[test]
    // fn index() {
    //     let client = Client::tracked(super::rocket()).expect("valid rocket instance");
    //     let response = client.get(format!("/api{}", uri!(super::index))).dispatch();
    //
    //     assert_eq!(response.status(), Status::Ok);
    //     assert_eq!(response.into_string(), Some(String::from("Hello, world!")));
    // }
}
