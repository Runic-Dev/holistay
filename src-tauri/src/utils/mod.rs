#[must_use] 
pub fn get_err_message(err: sqlx::Error) -> String {
    match err {
        sqlx::Error::Database(database_err) if database_err.is_unique_violation() => {
            "A user with this username already exists".to_string()
        },
        // sqlx::Error::Configuration(_) => todo!(),
        // sqlx::Error::Io(_) => todo!(),
        // sqlx::Error::Tls(_) => todo!(),
        // sqlx::Error::Protocol(_) => todo!(),
        // sqlx::Error::RowNotFound => todo!(),
        // sqlx::Error::TypeNotFound { type_name } => todo!(),
        // sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
        // sqlx::Error::ColumnNotFound(_) => todo!(),
        // sqlx::Error::ColumnDecode { index, source } => todo!(),
        // sqlx::Error::Decode(_) => todo!(),
        // sqlx::Error::AnyDriverError(_) => todo!(),
        // sqlx::Error::PoolTimedOut => todo!(),
        // sqlx::Error::PoolClosed => todo!(),
        // sqlx::Error::WorkerCrashed => todo!(),
        // sqlx::Error::Migrate(_) => todo!(),
        _ => err.to_string(),
    }
}
