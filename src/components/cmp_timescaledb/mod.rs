//! Компонент сохранения данных в БД TimescaleDB.
//!
//! Для сохранения используется библиотека [sqlx](https://crates.io/crates/sqlx)
//!
//! ## Разработка
//!
//! Запустить тестовую базу данных из файла docker compose - см README в корне

mod component;
mod config;
mod error;
mod fn_process;
mod model;

/// Компонент сохранения сообщений в TimescaleDB
pub use {component::Cmp, config::Config};
