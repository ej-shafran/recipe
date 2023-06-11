use hmac::{Hmac, Mac};
use jwt::{Claims, RegisteredClaims, SignWithKey, VerifyWithKey};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::serde::{json, Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserID(String);

struct EnvVariables {
    key: Hmac<Sha256>,
    auth_cookie_name: String,
}

impl EnvVariables {
    #[inline]
    pub fn parse() -> Self {
        let key = env::var("SECRET_KEY").expect("No SECRET_KEY env variable set");
        let key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes()).unwrap();
        let auth_cookie_name = env::var("AUTH_COOKIE").expect("No AUTH_COOKIE env variable set");

        Self {
            key,
            auth_cookie_name,
        }
    }
}

fn from_request_impl(req: &Request<'_>) -> Option<UserID> {
    let env_variables = EnvVariables::parse();

    let auth_cookie = req.cookies().get(&env_variables.auth_cookie_name)?.value();

    let token: Claims = auth_cookie.verify_with_key(&env_variables.key).ok()?;

    let expiration = token.registered.expiration?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now should be after 1970")
        .as_secs();

    if now >= expiration {
        return None;
    }

    match token.private.get("id") {
        Some(json::Value::String(id)) => Some(UserID(id.to_string())),
        _ => None,
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for UserID {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match from_request_impl(req) {
            None => request::Outcome::Failure((Status::Unauthorized, ())),
            Some(user_id) => request::Outcome::Success(user_id),
        }
    }
}

impl Into<String> for UserID {
    fn into(self) -> String {
        self.0
    }
}

const WEEK_IN_SECONDS: u64 = 604800;

impl UserID {
    pub fn add_to_cookie(id: &str, cookies: &CookieJar) {
        let env_variables = EnvVariables::parse();

        let expiration = SystemTime::now() + Duration::from_secs(WEEK_IN_SECONDS);

        let registered = RegisteredClaims {
            expiration: expiration
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .ok(),
            ..Default::default()
        };
        let mut claims = Claims::new(registered);
        claims.private.insert("id".into(), id.into());

        let token = claims.sign_with_key(&env_variables.key).expect("");

        cookies.add(Cookie::new(
            env_variables.auth_cookie_name,
            token.as_str().to_owned(),
        ));
    }
}
