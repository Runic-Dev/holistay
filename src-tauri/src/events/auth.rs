use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::models::{LoginRegisterAttempt, user::User};

pub async fn register_user(
    conn_pool: Pool<Sqlite>,
    register_attempt: LoginRegisterAttempt,
) -> Result<(), sqlx::Error> {
    match conn_pool.begin().await {
        Ok(mut tran) => {
            let id = Uuid::new_v4();

            if let Err(err) = sqlx::query("INSERT INTO user (id, username) VALUES (?,?)")
                .bind(id.clone().to_string())
                .bind(register_attempt.username.clone())
                .execute(&mut *tran)
                .await
            {
                println!("Error inserting user into user table: {:?}", err);
            }
            if let Err(err) =
                sqlx::query("INSERT INTO auth (id, username, password) VALUES (?,?,?)")
                    .bind(Uuid::new_v4().to_string())
                    .bind(register_attempt.username.clone())
                    .bind(register_attempt.password)
                    .execute(&mut *tran)
                    .await
            {
                println!("Error inserting user into auth table: {:?}", err);
            }

            match tran.commit().await {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn login_user(
    conn_pool: Pool<Sqlite>,
    login_attempt: LoginRegisterAttempt,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<Sqlite, User>("SELECT * FROM user INNER JOIN auth ON user.username = auth.username WHERE user.username = ? AND auth.password = ?")
        .bind(login_attempt.username)
        .bind(login_attempt.password)
        .fetch_one(&conn_pool)
        .await 
}


