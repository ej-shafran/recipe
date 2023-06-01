#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Recipe {
    pub id: u32,
    pub title: String,
    pub content: String,
}

