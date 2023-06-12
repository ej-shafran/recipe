use super::*;
use crate::user::{self, UserDTO};
use sqlx::MySqlPool;

fn get_dto(index: u32) -> RecipeDTO {
    RecipeDTO {
        title: format!("Title for {index}"),
        content: format!("Content for {index}"),
    }
}

async fn get_user_id(db: &mut PoolConnection<MySql>) -> String {
    let test_user = UserDTO {
        username: String::from("testing"),
        password: String::from("abcd"),
    };

    user::register(&test_user, db).await.unwrap()
}

#[sqlx::test]
#[ignore]
async fn creates_and_reads_details(pool: MySqlPool) {
    let mut db = pool.acquire().await.unwrap();

    let user_id = get_user_id(&mut db).await;
    let first_recipe = get_dto(0);

    let recipe_id = create_one(&first_recipe, &user_id, &mut db)
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
async fn reads_previews(pool: MySqlPool) {
    let mut db = pool.acquire().await.unwrap();

    let user_id = get_user_id(&mut db).await;

    let mut handles = Vec::new();

    for i in 1..=15 {
        let user_id = user_id.clone();
        let mut db = pool.acquire().await.unwrap();

        let recipe = get_dto(i);

        let handle = rocket::tokio::task::spawn(async move {
            create_one(&recipe, &user_id, &mut db).await.unwrap()
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    for i in 1..=3 {
        let previews = read_previews(i, 5, &mut db)
            .await
            .expect("could not read previews");

        assert_eq!(previews.results.len(), 5);
    }
}

#[sqlx::test]
#[ignore]
async fn deletes(pool: MySqlPool) {
    let mut db = pool.acquire().await.unwrap();

    let user_id = get_user_id(&mut db).await;
    let recipe = get_dto(0);

    let recipe_id = create_one(&recipe, &user_id, &mut db).await.unwrap();

    delete_one(recipe_id, &mut db)
        .await
        .expect("failed to delete recipe");

    let from_db = read_details(recipe_id, &mut db).await.unwrap_err();
    assert_eq!(from_db, Status::NotFound);
}
