use rocket::{http::Status, serde::Deserialize};
use sqlx::{pool::PoolConnection, MySql};

use crate::schema::{RecipeDetails, RecipePreview, User};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeDTO {
    pub title: String,
    pub content: String,
}

pub async fn create(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::{self, UserDTO};
    use sqlx::MySqlPool;

    #[sqlx::test]
    #[ignore]
    async fn creates_and_reads_details(pool: MySqlPool) {
        let mut db = pool.acquire().await.expect("invalid connection");

        let test_user = UserDTO {
            username: String::from("testing"),
            password: String::from("abcd"),
        };
        let user_id = user::register(&test_user, &mut db).await.unwrap();

        let first_recipe = RecipeDTO {
            title: String::from("Gramama's Golden Cookies"),
            content: String::from("My grandma used to love baking us cookies..."),
        };

        let recipe_id = create(&first_recipe, &user_id, &mut db)
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
    async fn creates_and_reads_previews(pool: MySqlPool) {
        let mut db = pool.acquire().await.expect("invalid connection");

        let test_user = UserDTO {
            username: String::from("testing"),
            password: String::from("abcd"),
        };
        let user_id = user::register(&test_user, &mut db).await.unwrap();

        let mut handles = Vec::new();

        for i in 1..=15 {
            let user_id = user_id.clone();
            let mut db = pool.acquire().await.expect("invalid connection");

            let recipe = RecipeDTO {
                title: format!("Recipe {}", i),
                content: format!("Content for Recipe {}", i),
            };

            let handle = rocket::tokio::task::spawn(async move {
                create(&recipe, &user_id, &mut db)
                    .await
                    .expect("failed to create recipe");
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let preview_first_page = read_previews(1, 10, &mut db)
            .await
            .expect("could not read previews");

        assert_eq!(preview_first_page.len(), 10);

        let preview_second_page = read_previews(2, 10, &mut db)
            .await
            .expect("could not read previews");

        assert_eq!(preview_second_page.len(), 5);
    }
}
