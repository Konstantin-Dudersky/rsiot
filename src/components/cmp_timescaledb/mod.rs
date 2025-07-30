//! Компонент сохранения данных в БД TimescaleDB.
//!
//! Для сохранения используется библиотека [sqlx](https://crates.io/crates/sqlx)
//!
//! Пример создания компонента:
#![doc = include_str!("../../../examples/cmp_timescaledb/config_timescaledb.rs")]

mod component;
mod config;
mod error;
mod fn_process;
mod helpers;
mod model;
mod tasks;

pub use {
    component::Cmp,
    config::Config,
    error::Error,
    helpers::*,
    model::{AggType, Row},
};

type Result<T> = std::result::Result<T, Error>;

// async fn save_row_in_db(row: Row, pool: Pool<Postgres>) -> Result<()> {
//     debug!("Save row in database: {:?}", row);
//     query(
//         r#"
// INSERT INTO raw
// VALUES ($1, $2, $3, $4, $5, $6, $7)
// ON CONFLICT (time, entity, attr, agg) DO UPDATE
//     SET value = excluded.value,
//          aggts = excluded.aggts,
//          aggnext = excluded.aggnext;"#,
//     )
//     .bind(row.time)
//     .bind(&row.entity)
//     .bind(&row.attr)
//     .bind(row.value)
//     .bind(&row.agg)
//     .bind(row.aggts)
//     .bind(&row.aggnext)
//     .execute(&pool)
//     .await?;
//     Ok(())
// }
