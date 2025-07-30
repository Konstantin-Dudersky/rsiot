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
