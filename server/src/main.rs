#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_db_pools::{Connection, Database};

pub mod schema;

#[derive(Database)]
#[database("recipe_db")]
pub struct DB(sqlx::MySqlPool);

#[get("/")]
pub async fn index(mut conn: Connection<DB>) -> String {
    sqlx::query!("SELECT 'Hello, world!' as result;")
        .fetch_one(&mut *conn)
        .await
        .unwrap()
        .result
}

#[launch]
pub fn rocket() -> Rocket<Build> {
    dotenvy::dotenv().expect("cannot find dotenv file");

    rocket::build()
        .mount("/api", routes![index])
        .attach(DB::init())
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
