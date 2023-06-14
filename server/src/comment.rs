use crate::schema::{Comment, Paginated, User};
use rocket::{http::Status, serde::Deserialize};
use sqlx::{pool::PoolConnection, Connection, MySql};

#[cfg(test)]
mod tests;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CommentDTO {
    pub content: String,
}

pub async fn create_one(
    comment: &CommentDTO,
    user_id: &str,
    recipe_id: u64,
    db: &mut PoolConnection<MySql>,
) -> Result<u64, Status> {
    let query = sqlx::query!(
        "INSERT INTO comment (content, user_id, recipe_id) VALUES (?, ?, ?);",
        comment.content,
        user_id,
        recipe_id
    );

    query
        .execute(db)
        .await
        .or(Err(Status::InternalServerError))
        .map(|response| response.last_insert_id())
}

pub async fn read_many(
    recipe_id: u64,
    page: u32,
    limit: u8,
    db: &mut PoolConnection<MySql>,
) -> Result<Paginated<Comment>, Status> {
    let mut transaction = db.begin().await.or(Err(Status::InternalServerError))?;

    let query = sqlx::query!(
        "SELECT 
            c.id AS `id: u64`,
            c.content AS content,
            u.id AS poster_id,
            u.username AS `poster_name!`
         FROM (SELECT id, content, recipe_id, user_id FROM comment LIMIT ? OFFSET ?) AS c
         JOIN user AS u
         ON u.id = c.user_id
         WHERE recipe_id = ?;",
        limit,
        (page - 1) * (limit as u32),
        recipe_id
    );

    let rows = query
        .fetch_all(&mut transaction)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => Status::NotFound,
            _ => Status::InternalServerError,
        })?;

    let query = sqlx::query!("SELECT COUNT(id) AS `count: u32` FROM comment;");

    let count = query
        .fetch_one(&mut transaction)
        .await
        .or(Err(Status::InternalServerError))?
        .count;

    let results: Vec<Comment> = rows
        .into_iter()
        .map(|row| Comment {
            id: row.id,
            content: row.content,
            poster: User {
                username: row.poster_name,
                id: row.poster_id,
            },
        })
        .collect();

    Ok(Paginated {
        results,
        next_page: if count <= page * (limit as u32) {
            None
        } else {
            Some(page + 1)
        },
    })
}
