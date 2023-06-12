use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipePreview {
    pub id: u64,
    pub title: String,
    pub poster: User,
    #[serde(rename = "commentCount")]
    pub comment_count: u64,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeDetails {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub poster: User,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Comment {
    pub id: u32,
    pub content: String,
    pub poster: User,
}
