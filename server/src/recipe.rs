use crate::schema::{RecipeDetails, RecipePreview, User};
use rocket::{http::Status, serde::Deserialize};
use sqlx::{pool::PoolConnection, MySql};

pub mod routes;

#[cfg(test)]
mod tests;

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
