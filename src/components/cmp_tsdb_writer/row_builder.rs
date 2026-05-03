use sqlx::types::time::OffsetDateTime;
use time::format_description::well_known::Iso8601;

use super::Error;

fn row_to_sql(time: &str, values: &[impl AsRef<str>]) -> String {
    let values = values
        .iter()
        .map(|v| v.as_ref())
        .collect::<Vec<_>>()
        .join(", ");
    format!("('{}', {})", time, values)
}

/// Создает строку для записи в БД.
pub fn row_with_ts(time: &OffsetDateTime, values: &[impl AsRef<str>]) -> Result<String, Error> {
    let time = time.format(&Iso8601::DEFAULT)?;
    let sql = row_to_sql(&time, values);
    Ok(sql)
}

/// Создает строку для записи в БД. Время берется из локального времени.
pub fn row_without_ts(values: &[impl AsRef<str>]) -> Result<String, Error> {
    let time = OffsetDateTime::now_local()?.format(&Iso8601::DEFAULT)?;
    let sql = row_to_sql(&time, values);
    Ok(sql)
}
