use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserID(String);

#[inline]
fn get_secret_key() -> Hmac<Sha256> {
    let secret_key = env::var("SECRET_KEY").expect("No SECRET_KEY env variable set");
    Hmac::new_from_slice(&secret_key.as_bytes()).unwrap()
}

#[async_trait]
impl<'r> FromRequest<'r> for UserID {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_cookie_name = env::var("AUTH_COOKIE").expect("No AUTH_COOKIE env variable set");

        let cookies = req.cookies();
        let auth_cookie = cookies.get(&auth_cookie_name);

        match auth_cookie {
            Some(auth_cookie) => {
                let secret_key = get_secret_key();

                let auth_cookie = auth_cookie.value();
                let user_id: UserID = match auth_cookie.verify_with_key(&secret_key) {
                    Err(err) => {
                        dbg!(err);
                        return Outcome::Failure((Status::Unauthorized, ()));
                    }
                    Ok(value) => value,
                };

                Outcome::Success(user_id)
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

impl UserID {
    pub fn sign(value: String) -> String {
        let secret_key = get_secret_key();
        UserID(value).sign_with_key(&secret_key).unwrap()
    }
}
