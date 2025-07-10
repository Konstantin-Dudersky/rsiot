//! Компонент сохранения данных в БД TimescaleDB.
//!
//! Для сохранения используется библиотека [sqlx](https://crates.io/crates/sqlx)
//!
//! ## Разработка
//!
//! Запустить тестовую базу данных из файла docker compose - см README в корне
//!
//! TODO - добавить в target_config после устранения ошибок

mod component;
mod config;
mod error;
mod fn_process;
mod model;

pub use {
    component::Cmp,
    config::Config,
    model::{AggType, Row},
};
