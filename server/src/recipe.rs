use rocket::{http::Status, serde::Deserialize};
use sqlx::{pool::PoolConnection, MySql};

use crate::schema::{RecipeDetails, RecipePreview, User};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeDTO {
    pub title: String,
    pub content: String,
}

pub async fn create_one(
    recipe: &RecipeDTO,
    poster_id: &str,
    db: &mut PoolConnection<MySql>,
) -> Result<u64, Status> {
    let query = sqlx::query!(
        "INSERT INTO recipe (title, content, poster_id) VALUES (?, ?, ?);",
        recipe.title,
        recipe.content,
        poster_id
    );

    match query.execute(&mut *db).await {
        Ok(result) => Ok(result.last_insert_id()),
        _ => Err(Status::InternalServerError),
    }
}

pub async fn read_details(
    id: u64,
    db: &mut PoolConnection<MySql>,
) -> Result<RecipeDetails, Status> {
    let query = sqlx::query!(
        "SELECT 
            r.id AS `id: u64`,
            r.title AS title, 
            r.content AS content, 
            u.id AS poster_id, 
            u.username AS `poster_name!` 
        FROM recipe AS r 
        JOIN user AS u 
        ON r.poster_id = u.id 
        WHERE r.id = ?;",
        id
    );

    let row = query.fetch_one(&mut *db).await.map_err(|err| match err {
        sqlx::Error::RowNotFound => Status::NotFound,
        _ => Status::InternalServerError,
    })?;

    Ok(RecipeDetails {
        id: row.id,
        title: row.title,
        content: row.content,
        poster: User {
            id: row.poster_id,
            username: row.poster_name,
        },
    })
}

pub async fn read_previews(
    page: u32,
    limit: u32,
    db: &mut PoolConnection<MySql>,
) -> Result<Vec<RecipePreview>, Status> {
    let query = sqlx::query!(
        "SELECT 
            r.id AS `id: u64`,
            r.title AS title,
            u.id AS poster_id,
            u.username AS `poster_name!`,
            COUNT(c.id) AS `comment_count: u64`
         FROM (SELECT id, title, poster_id FROM recipe LIMIT ? OFFSET ?) AS r
         JOIN user AS u
         ON r.poster_id = u.id
         LEFT JOIN comment AS c
         ON r.id = c.recipe_id
         GROUP BY r.id;",
        limit,
        (page - 1) * limit,
    );

    let rows = query
        .fetch_all(&mut *db)
        .await
        .or(Err(Status::InternalServerError))?;

    Ok(rows
        .into_iter()
        .map(|row| RecipePreview {
            id: row.id,
            title: row.title,
            poster: User {
                id: row.poster_id,
                username: row.poster_name,
            },
            comment_count: row.comment_count,
        })
        .collect())
}

pub async fn delete_one(id: u64, db: &mut PoolConnection<MySql>) -> Result<(), Status> {
    let query = sqlx::query!("DELETE FROM recipe WHERE id = ?", id);

    match query.execute(&mut *db).await {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::{self, UserDTO};
    use sqlx::MySqlPool;

    fn get_dto(index: u32) -> RecipeDTO {
        RecipeDTO {
            title: format!("Title for {index}"),
            content: format!("Content for {index}"),
        }
    }

    async fn get_user_id(db: &mut PoolConnection<MySql>) -> String {
        let test_user = UserDTO {
            username: String::from("testing"),
            password: String::from("abcd"),
        };

        user::register(&test_user, db).await.unwrap()
    }

    #[sqlx::test]
    #[ignore]
    async fn creates_and_reads_details(pool: MySqlPool) {
        let mut db = pool.acquire().await.unwrap();

        let user_id = get_user_id(&mut db).await;
        let first_recipe = get_dto(0);

        let recipe_id = create_one(&first_recipe, &user_id, &mut db)
            .await
            .expect("failed to create recipe");

        let recipe_from_db = read_details(recipe_id, &mut db)
            .await
            .expect("recipe id not found in database");

        assert_eq!(recipe_from_db.title, first_recipe.title);
        assert_eq!(recipe_from_db.content, first_recipe.content);
    }

    #[sqlx::test]
    #[ignore]
    async fn reads_previews(pool: MySqlPool) {
        let mut db = pool.acquire().await.unwrap();

        let user_id = get_user_id(&mut db).await;

        let mut handles = Vec::new();

        for i in 1..=15 {
            let user_id = user_id.clone();
            let mut db = pool.acquire().await.unwrap();

            let recipe = get_dto(i);

            let handle = rocket::tokio::task::spawn(async move {
                create_one(&recipe, &user_id, &mut db).await.unwrap()
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        for i in 1..=3 {
            let previews = read_previews(i, 5, &mut db)
                .await
                .expect("could not read previews");

            assert_eq!(previews.len(), 5);
        }
    }

    #[sqlx::test]
    #[ignore]
    async fn deletes(pool: MySqlPool) {
        let mut db = pool.acquire().await.unwrap();

        let user_id = get_user_id(&mut db).await;
        let recipe = get_dto(0);

        let recipe_id = create_one(&recipe, &user_id, &mut db).await.unwrap();

        delete_one(recipe_id, &mut db)
            .await
            .expect("failed to delete recipe");

        let from_db = read_details(recipe_id, &mut db).await.unwrap_err();
        assert_eq!(from_db, Status::NotFound);
    }
}
