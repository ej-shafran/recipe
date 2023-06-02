use crate::DB;
use anyhow::anyhow;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn register(password: &str, username: &str, db: DB) -> anyhow::Result<String> {
        let hashed = bcrypt::hash(password, 10)?;
        let id = Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO `user` (id, password, username) VALUES (?, ?, ?);",
            id,
            hashed,
            username
        )
        .execute(&*db)
        .await?;

        Ok(id)
    }

    pub async fn login(password: &str, username: &str, db: DB) -> anyhow::Result<()> {
        sqlx::query!(
            "SELECT `password` FROM `user` WHERE username = ?;",
            username
        )
        .fetch_one(&*db)
        .await?
        .password
        .and_then(|hashed| bcrypt::verify(password, &hashed).ok())
        .ok_or(anyhow!("Invalid credentials."))
        .map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::MySqlPool;

    #[sqlx::test]
    #[ignore]
    async fn register_and_login(pool: MySqlPool) {
        let id = User::register("abcd1234", "evyatar_shafran", DB(pool.clone()))
            .await
            .expect("valid addition to db");

        dbg!(id);

        assert!(
            User::login("abcd1234", "evyatar_shafran", DB(pool.clone()))
                .await
                .is_ok(),
            "login with correct credentials should work"
        );
        assert!(
            User::login("abcd1234", "some_other", DB(pool.clone()))
                .await
                .is_err(),
            "login when user does not exist fails"
        );
        assert!(
            User::login("abc1234", "evyatar_shafra", DB(pool))
                .await
                .is_err(),
            "login with incorrect password fails"
        );
    }
}
