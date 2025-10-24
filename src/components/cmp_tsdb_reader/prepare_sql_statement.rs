use time::{OffsetDateTime, format_description::well_known::Iso8601};

use super::Error;

pub fn prepare_sql_statement(
    time_begin: OffsetDateTime,
    time_end: OffsetDateTime,
    entity: &str,
    attribute: &str,
) -> Result<String, Error> {
    let time_start = time_begin.format(&Iso8601::DEFAULT)?;
    let time_stop = time_end.format(&Iso8601::DEFAULT)?;

    let sql = format!(
        r#"SELECT "time", value
FROM raw
WHERE (
    "time" BETWEEN '{time_start}' AND '{time_stop}'
	AND entity = '{entity}'
	AND attr = '{attribute}'
)
ORDER BY "time" ASC;"#
    );
    Ok(sql)
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let time_start = datetime!(2025-08-19 07:20:00.000+03);
        let time_stop = datetime!(2025-08-19 07:20:10.000+03);
        let entity = "accelerometer";
        let attr = "accel_x";

        let sql_test = prepare_sql_statement(time_start, time_stop, entity, attr)?;

        let sql = r#"SELECT "time", value
FROM raw
WHERE (
    "time" BETWEEN '2025-08-19T07:20:00.000000000+03:00' AND '2025-08-19T07:20:10.000000000+03:00'
	AND entity = 'accelerometer'
	AND attr = 'accel_x'
)
ORDER BY "time" ASC;"#;

        assert_eq!(sql_test, sql);
        Ok(())
    }
}
