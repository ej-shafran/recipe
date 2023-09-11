use crate::{auth, schema, DB};
use rocket::{http::Status, serde::json::Json, Route};
use rocket_db_pools::Connection;

pub fn index() -> Vec<Route> {
    routes![post, get_previews, get_details, get_count, delete]
}

#[post("/", data = "<recipe>")]
pub async fn post(
    recipe: Json<super::RecipeDTO>,
    user_id: auth::UserID,
    mut db: Connection<DB>,
) -> Result<Json<u64>, Status> {
    let user_id: String = user_id.into();

    super::create_one(&recipe, &user_id, &mut db)
        .await
        .map(Json::from)
}

#[get("/previews?<page>&<limit>")]
pub async fn get_previews(
    page: u32,
    limit: u8,
    mut db: Connection<DB>,
) -> Result<Json<schema::Paginated<schema::RecipePreview>>, Status> {
    super::read_previews(page, limit, &mut db)
        .await
        .map(Json::from)
}

#[get("/<id>")]
pub async fn get_details(
    id: u64,
    mut db: Connection<DB>,
) -> Result<Json<schema::RecipeDetails>, Status> {
    super::read_details(id, &mut db).await.map(Json::from)
}

#[get("/count")]
pub async fn get_count(mut db: Connection<DB>/*, _user: auth::UserID*/) -> Result<Json<u32>, Status> {
    super::read_count(&mut db).await.map(Json::from)
}

#[delete("/<id>")]
pub async fn delete(id: u64, _user: auth::UserID, mut db: Connection<DB>) -> Result<(), Status> {
    super::delete_one(id, &mut db).await
}
