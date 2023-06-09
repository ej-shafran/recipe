use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlDatabaseError;
use sqlx::pool::PoolConnection;
use sqlx::MySql;
use uuid::Uuid;

#[cfg(test)]
mod tests;

pub mod routes;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDTO {
    pub username: String,
    pub password: String,
}

pub async fn register(user: &UserDTO, db: &mut PoolConnection<MySql>) -> Result<String, Status> {
    let hashed = bcrypt::hash(&user.password, 10).or(Err(Status::InternalServerError))?;
    let id = Uuid::new_v4().to_string();

    let query = sqlx::query!(
        "INSERT INTO `user` (id, password, username) VALUES (?, ?, ?);",
        id,
        hashed,
        &user.username
    );

    query.execute(&mut *db).await.map_err(|err| match err {
        sqlx::Error::Database(err) if is_duplicate_key_err(err.downcast_ref()) => {
            Status::Unauthorized
        }
        _ => Status::InternalServerError,
    })?;

    Ok(id)
}

pub async fn login(user: &UserDTO, db: &mut PoolConnection<MySql>) -> Result<String, Status> {
    let query = sqlx::query!(
        "SELECT `password` as `password!`, `id` FROM `user` WHERE username = ?;",
        &user.username
    );

    let result = query.fetch_one(&mut *db).await.map_err(|err| match err {
        sqlx::Error::RowNotFound => Status::Unauthorized,
        _ => Status::InternalServerError,
    })?;

    match bcrypt::verify(&user.password, &result.password) {
        Ok(true) => Ok(result.id),
        Ok(false) => Err(Status::Unauthorized),
        Err(_) => Err(Status::InternalServerError),
    }
}

const DUPLICATE_KEY_CODE: &str = "23000";

#[inline]
fn is_duplicate_key_err(err: &MySqlDatabaseError) -> bool {
    err.code() == Some(DUPLICATE_KEY_CODE)
}
