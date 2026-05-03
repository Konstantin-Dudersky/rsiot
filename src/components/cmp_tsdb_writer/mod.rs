//! Компонент сохранения данных в БД TimescaleDB.
//!
//! Для сохранения используется библиотека [sqlx](https://crates.io/crates/sqlx)
//!
//! Пример создания компонента:
#![doc = include_str!("../../../examples/cmp_tsdb_writer/config_tsdb.rs")]

mod component;
mod config;
mod error;
mod fn_process;
pub mod helpers;
mod query_stat;
mod row_builder;
mod tasks;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::{Config, ConfigTable, ConfigTableField, ConfigTableFieldType},
    error::Error,
    query_stat::QueryStat,
    row_builder::{row_with_ts, row_without_ts},
};

type Result<T> = std::result::Result<T, Error>;
