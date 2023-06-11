#[macro_use]
extern crate rocket;

pub mod auth;
pub mod recipe;
pub mod schema;
pub mod user;

use auth::UserID;
use recipe::RecipeDTO;
use rocket::http::CookieJar;
use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket::{Build, Rocket};
use rocket_db_pools::{Connection, Database};
use schema::{RecipeDetails, RecipePreview};
use user::UserDTO;

#[derive(Database)]
#[database("recipe_db")]
pub struct DB(sqlx::MySqlPool);

#[launch]
fn launch() -> _ {
    rocket()
}

pub fn rocket() -> Rocket<Build> {
    dotenvy::dotenv().expect("cannot find dotenv file");

    rocket::build()
        .mount(
            "/api",
            routes![
                login,
                register,
                create_recipe,
                recipe_details,
                recipe_previews
            ],
        )
        .attach(DB::init())
        .attach(AdHoc::try_on_ignite("Run migrations", |rocket| async {
            if let Some(db) = DB::fetch(&rocket) {
                if let Err(err) = sqlx::migrate!().run(&db.0).await {
                    dbg!(err);
                    return Err(rocket);
                }
                Ok(rocket)
            } else {
                dbg!("Failed to fetch DB from rocket");
                Err(rocket)
            }
        }))
}

#[post("/login", data = "<user>")]
pub async fn login(
    user: Json<UserDTO>,
    mut db: Connection<DB>,
    cookies: &CookieJar<'_>,
) -> Result<(), Status> {
    let id = user::login(&user.into_inner(), &mut db).await?;

    UserID::add_to_cookie(&id, cookies);

    Ok(())
}

#[post("/register", data = "<user>")]
pub async fn register(
    user: Json<UserDTO>,
    mut db: Connection<DB>,
    cookies: &CookieJar<'_>,
) -> Result<(), Status> {
    let id = user::register(&user.into_inner(), &mut db).await?;

    UserID::add_to_cookie(&id, cookies);

    Ok(())
}

#[post("/recipes/create", data = "<recipe>")]
pub async fn create_recipe(
    recipe: Json<RecipeDTO>,
    user_id: UserID,
    mut db: Connection<DB>,
) -> Result<Json<u64>, Status> {
    let user_id: String = user_id.into();

    let id = recipe::create(&recipe, &user_id, &mut db).await?;

    Ok(id.into())
}

#[get("/recipes/previews?<page>&<limit>")]
pub async fn recipe_previews(
    page: u32,
    limit: u32,
    mut db: Connection<DB>,
) -> Result<Json<Vec<RecipePreview>>, Status> {
    recipe::read_previews(page, limit, &mut db)
        .await
        .map(|value| value.into())
}

#[get("/recipes/<id>")]
pub async fn recipe_details(
    id: u64,
    mut db: Connection<DB>,
) -> Result<Json<RecipeDetails>, Status> {
    recipe::read_details(id, &mut db)
        .await
        .map(|value| value.into())
}
