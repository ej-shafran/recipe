use crate::{auth, schema, DB};
use rocket::{http::Status, serde::json::Json, Route};
use rocket_db_pools::Connection;

pub fn index() -> Vec<Route> {
    routes![post, get]
}

#[post("/<recipe_id>", data = "<comment>")]
pub async fn post(
    recipe_id: u64,
    comment: Json<super::CommentDTO>,
    user_id: auth::UserID,
    mut db: Connection<DB>,
) -> Result<Json<u64>, Status> {
    let user_id: String = user_id.into();

    super::create_one(&comment, &user_id, recipe_id, &mut db)
        .await
        .map(Json::from)
}

#[get("/<recipe_id>?<page>&<limit>")]
pub async fn get(
    recipe_id: u64,
    page: u32,
    limit: u8,
    mut db: Connection<DB>,
) -> Result<Json<schema::Paginated<super::Comment>>, Status> {
    super::read_many(recipe_id, page, limit, &mut db)
        .await
        .map(Json::from)
}
