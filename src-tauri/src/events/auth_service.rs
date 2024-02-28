use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use tokio::sync::MutexGuard;

use crate::models::user::User;

use super::requests::LoginRegisterRequest;

pub async fn register_user(
    conn_pool: MutexGuard<'_, Pool<Sqlite>>,
    register_attempt: LoginRegisterRequest,
) -> Result<(), sqlx::Error> {
    let mut tran = conn_pool.begin().await?;

    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO user (id, username) VALUES (?,?)")
        .bind(id.clone().to_string())
        .bind(register_attempt.username.clone())
        .execute(&mut *tran)
        .await?;

    sqlx::query("INSERT INTO auth (id, username, password) VALUES (?,?,?)")
        .bind(Uuid::new_v4().to_string())
        .bind(register_attempt.username.clone())
        .bind(register_attempt.password)
        .execute(&mut *tran)
        .await?;

    tran.commit().await
}

pub async fn login_user(pool_lock: MutexGuard<'_, Pool<Sqlite>>, login_attempt: LoginRegisterRequest) -> Result<User, sqlx::Error> {
    sqlx::query_as::<Sqlite, User>("SELECT * FROM user INNER JOIN auth ON user.username = auth.username WHERE user.username = ? AND auth.password = ?")
        .bind(login_attempt.username)
        .bind(login_attempt.password)
        .fetch_one(&*pool_lock)
    .await
}
