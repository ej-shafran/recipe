#[derive(Debug)]
pub struct Recipe {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Comment {
    pub id: u32,
    pub content: String,
}
