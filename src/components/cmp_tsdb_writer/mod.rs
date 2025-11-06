//! Компонент сохранения данных в БД TimescaleDB.
//!
//! Для сохранения используется библиотека [sqlx](https://crates.io/crates/sqlx)
//!
//! Пример создания компонента:
#![doc = include_str!("../../../examples/cmp_tsdb/config_tsdb.rs")]

mod component;
mod config;
mod error;
mod fn_process;
mod helpers;
mod query_stat;
mod row;
mod tasks;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
    helpers::*,
    query_stat::QueryStat,
    row::{RowBuilder, RowPrj, RowPrjHst, RowPrjHstSvc},
};

type Result<T> = std::result::Result<T, Error>;
