#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{http::Status, local::blocking::Client};

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(format!("/api{}", uri!(index))).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(String::from("Hello, world!")));
    }
}
