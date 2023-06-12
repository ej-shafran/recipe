use super::*;
use sqlx::MySqlPool;

#[sqlx::test]
#[ignore]
async fn register_and_login(pool: MySqlPool) {
    let mut db = pool.acquire().await.expect("valid connection");

    let correct_user = UserDTO {
        username: "evyatar_shafran".to_string(),
        password: "abcd1234".to_string(),
    };

    let incorrect_username = UserDTO {
        password: correct_user.password.clone(),
        username: "wrong username".to_string(),
    };

    let incorrect_password = UserDTO {
        password: "wrong password".to_string(),
        username: correct_user.username.clone(),
    };

    register(&correct_user, &mut db)
        .await
        .expect("valid addition to db");

    assert!(
        login(&correct_user, &mut db).await.is_ok(),
        "login with correct credentials should work"
    );
    assert!(
        login(&incorrect_username, &mut db).await.is_err(),
        "login when user does not exist fails"
    );
    assert!(
        login(&incorrect_password, &mut db).await.is_err(),
        "login with incorrect password fails"
    );
}
