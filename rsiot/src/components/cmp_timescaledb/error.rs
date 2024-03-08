use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum Error {
    SqlxError(SqlxError),
}

impl From<SqlxError> for Error {
    fn from(value: SqlxError) -> Self {
        Self::SqlxError(value)
    }
}
