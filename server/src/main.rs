#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_db_pools::{Connection, Database};

pub mod schema;
pub mod user;

#[derive(Database)]
#[database("recipe_db")]
pub struct DB(sqlx::MySqlPool);

#[get("/")]
async fn index(mut conn: Connection<DB>) -> String {
    sqlx::query!("SELECT 'Hello, world!' as result;")
        .fetch_one(&mut *conn)
        .await
        .unwrap()
        .result
}

#[launch]
pub fn rocket() -> _ {
    dotenvy::dotenv().expect("cannot find dotenv file");

    rocket::build()
        .mount("/api", routes![index])
        .attach(DB::init())
        .attach(AdHoc::try_on_ignite("Run migrations", |rocket| async {
            if let Some(db) = DB::fetch(&rocket) {
                if let Err(_) = sqlx::migrate!().run(&db.0).await {
                    return Err(rocket);
                }
                Ok(rocket)
            } else {
                Err(rocket)
            }
        }))
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::{http::Status, local::blocking::Client};

    #[test]
    fn index() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");
        let response = client.get(format!("/api{}", uri!(super::index))).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(String::from("Hello, world!")));
    }
}
