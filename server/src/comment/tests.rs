use super::*;
use crate::{recipe, user};
use sqlx::MySqlPool;

async fn get_recipe_id(db: &mut PoolConnection<MySql>) -> (u64, String) {
    let test_user = user::UserDTO {
        username: String::from("testing"),
        password: String::from("abcd"),
    };

    let user_id = user::register(&test_user, db).await.unwrap();

    let test_recipe = recipe::RecipeDTO {
        title: String::from("Title for test recipe"),
        content: String::from("Content for test recipe"),
    };

    (
        recipe::create_one(&test_recipe, &user_id, db)
            .await
            .unwrap(),
        user_id,
    )
}

#[sqlx::test]
#[ignore]
#[serial_test::serial]
async fn reads_previews(pool: MySqlPool) {
    let mut db = pool.acquire().await.unwrap();
    let (recipe_id, user_id) = get_recipe_id(&mut db).await;

    let test_comment = CommentDTO {
        content: String::from("Wow this recipe is great!"),
    };

    create_one(&test_comment, &user_id, recipe_id, &mut db)
        .await
        .expect("failed to create comment");

    let comments_from_db = read_many(recipe_id, 1, 10, &mut db)
        .await
        .expect("failed to read the comments for the recipe");

    assert_eq!(
        vec![test_comment.content],
        comments_from_db
            .results
            .into_iter()
            .map(|comment| comment.content)
            .collect::<Vec<String>>()
    );
}
