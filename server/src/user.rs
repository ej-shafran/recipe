use crate::DB;
use anyhow::anyhow;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;
use sqlx::pool::PoolConnection;
use sqlx::MySql;
use uuid::Uuid;

pub struct User;

const DUPLICATE_KEY_CODE: &str = "23000";

impl User {
    pub async fn register(
        password: &str,
        username: &str,
        db: &mut PoolConnection<MySql>,
    ) -> anyhow::Result<String> {
        let hashed = bcrypt::hash(password, 10)?;
        let id = Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO `user` (id, password, username) VALUES (?, ?, ?);",
            id,
            hashed,
            username
        )
        .execute(&mut *db)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(err) => {
                let mysql_err = err.downcast_ref::<sqlx::mysql::MySqlDatabaseError>();

                if mysql_err.code() == Some(DUPLICATE_KEY_CODE) {
                    anyhow!("Duplicate User.")
                } else {
                    anyhow!(err)
                }
            }
            err => anyhow!(err),
        })?;

        Ok(id)
    }

    pub async fn login(
        password: &str,
        username: &str,
        db: &mut PoolConnection<MySql>,
    ) -> anyhow::Result<String> {
        let result = sqlx::query!(
            "SELECT `password`, `id` FROM `user` WHERE username = ?;",
            username
        )
        .fetch_one(&mut *db)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => anyhow!("Invalid Credentials."),
            err => anyhow!(err),
        })?;

        result
            .password
            .filter(|hashed| bcrypt::verify(password, &hashed).ok() == Some(true))
            .ok_or(anyhow!("Invalid Credentials."))?;

        Ok(result.id)
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDTO {
    pub username: String,
    pub password: String,
}

#[post("/register", data = "<user>")]
pub async fn register(user: Json<UserDTO>, mut db: Connection<DB>) -> (Status, String) {
    let result = User::register(&user.password, &user.username, &mut *db).await;

    match result {
        Ok(id) => (Status::Ok, id),
        Err(error) => {
            if error.downcast_ref() == Some(&"Duplicate User.") {
                let message = String::from("Duplicate User.");
                (Status::BadRequest, message)
            } else {
                dbg!(error);
                let message = String::from("Internal Server Error.");
                (Status::InternalServerError, message)
            }
        }
    }
}

#[post("/login", data = "<user>")]
pub async fn login(user: Json<UserDTO>, mut db: Connection<DB>) -> (Status, String) {
    let result = User::login(&user.password, &user.username, &mut *db).await;

    match result {
        Ok(id) => (Status::Ok, id),
        Err(error) => {
            if error.downcast_ref() == Some(&"Invalid Credentials.") {
                let message = String::from("Invalid Credentials.");
                (Status::Unauthorized, message)
            } else {
                dbg!(error);
                let message = String::from("Internal Server Error.");
                (Status::InternalServerError, message)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::MySqlPool;

    #[sqlx::test]
    #[ignore]
    async fn register_and_login(pool: MySqlPool) {
        let mut db = pool.acquire().await.expect("valid connection");
        let id = User::register("abcd1234", "evyatar_shafran", &mut db)
            .await
            .expect("valid addition to db");

        dbg!(id);

        assert!(
            User::login("abcd1234", "evyatar_shafran", &mut db)
                .await
                .is_ok(),
            "login with correct credentials should work"
        );
        assert!(
            User::login("abcd1234", "some_other", &mut db)
                .await
                .is_err(),
            "login when user does not exist fails"
        );
        assert!(
            User::login("abc1234", "evyatar_shafra", &mut db)
                .await
                .is_err(),
            "login with incorrect password fails"
        );
    }
}
