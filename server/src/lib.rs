#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

pub mod db {
    #[derive(Debug)]
    pub struct User {
        pub id: String,
        pub username: String,
        pub password: String,
    }

    #[derive(Debug)]
    pub struct Recipe {
        pub id: u32,
        pub title: String,
        pub content: String,
    }

    #[derive(Debug)]
    pub struct Comment {
        pub id: u32,
        pub content: String,
    }
}

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}

pub fn rocket() -> Rocket<Build> {
    rocket::build().mount("/api", routes![index])
}

#[cfg(test)]
mod tests {
    use rocket::{http::Status, local::blocking::Client};

    use super::rocket;

    #[test]
    fn index() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");
        let response = client.get(format!("/api{}", uri!(super::index))).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(String::from("Hello, world!")));
    }
}
