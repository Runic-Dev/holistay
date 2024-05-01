use sqlx::{Pool, Sqlite, prelude::FromRow, sqlite::SqliteRow, Row};
use uuid::Uuid;
use tokio::sync::MutexGuard;
use crate::models::requests::LoginRegisterRequest;

use crate::models::user::{User, UserRow};

pub async fn register_user(
    conn_pool: MutexGuard<'_, Pool<Sqlite>>,
    register_attempt: LoginRegisterRequest,
) -> Result<Uuid, sqlx::Error> {
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

    println!("Stay logged in: {}", register_attempt.stay_logged_in);

    if register_attempt.stay_logged_in {
        println!("Setting default user: {}", id.to_string());
        sqlx::query("UPDATE settings SET value = (?) WHERE setting = 'default_user'")
            .bind(id.to_string())
            .execute(&mut *tran)
        .await?;
    }

    match tran.commit().await {
        Ok(_) => Ok(id),
        Err(err) => Err(err),
    }
}

pub async fn login_user(conn_pool: MutexGuard<'_, Pool<Sqlite>>, login_attempt: LoginRegisterRequest) -> Result<User, sqlx::Error> {
    let mut tran = conn_pool.begin().await?;
    let user = sqlx::query_as::<Sqlite, User>("SELECT user.id, user.username FROM user INNER JOIN auth ON user.username = auth.username WHERE user.username = ? AND auth.password = ?")
        .bind(login_attempt.username)
        .bind(login_attempt.password)
        .fetch_one(&mut *tran).await?;

    if login_attempt.stay_logged_in {
        println!("Setting default user: {}", user.id.to_string());
        sqlx::query("UPDATE settings SET value = (?) WHERE setting = 'default_user'")
            .bind(user.id.to_string())
            .execute(&mut *tran)
        .await?;
    }

    tran.commit().await.map_or_else(|err| Err(err), |_| Ok(user))
}

pub async fn get_default_user(conn_pool: MutexGuard<'_, Pool<Sqlite>>) -> Result<Option<UserRow>, Box<dyn std::error::Error>> {

    let default_user_setting = sqlx::query_as::<Sqlite, DefaultUserSetting>("SELECT value FROM settings WHERE setting = 'default_user'")
        .fetch_one(&*conn_pool).await?;

    if let Some(user_id) = default_user_setting.value {
        println!("User id from settings table: {}", user_id);
        let user_row = sqlx::query_as::<Sqlite, UserRow>("SELECT id, username FROM user WHERE id = ?").bind(user_id).fetch_one(&*conn_pool).await?;
        Ok(Some(user_row))
    } else {
        Ok(None)
    }
}

struct DefaultUserSetting {
    pub value: Option<String>
}

impl FromRow<'_, SqliteRow> for DefaultUserSetting {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            value: row.try_get("value")?
        })
    }
}

impl FromRow<'_, SqliteRow> for UserRow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?
        })
    }
}
