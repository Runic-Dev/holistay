use sqlx::{Pool, Sqlite};

use uuid::Uuid;

use crate::RegisterAttempt;

pub async fn register_user(
    conn_pool: Pool<Sqlite>,
    register_attempt: RegisterAttempt,
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
                println!("Error on query 1: {:?}", err);
            }
            if let Err(err) =
                sqlx::query("INSERT INTO auth (id, username, password) VALUES (?,?,?)")
                    .bind(Uuid::new_v4().to_string())
                    .bind(register_attempt.username.clone())
                    .bind(register_attempt.password)
                    .execute(&mut *tran)
                    .await
            {
                println!("Error on query 2: {:?}", err);
            }

            match tran.commit().await {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}
