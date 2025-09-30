//! Опрос устройств Modbus, используя библиотеку
//! [tokio-modbus](https://crates.io/crates/tokio-modbus)
//!
//! Ссылки:
//!
//! - [Документация docs.rs](https://docs.rs/rsiot-modbus-client/latest/)
//!
//! - [Репозиторий
//!   GitHub](https://github.com/Konstantin-Dudersky/rsiot/tree/main/rsiot-modbus-client)
//!
//! - [Примеры](https://github.com/Konstantin-Dudersky/rsiot/tree/main/rsiot-modbus-client/examples)
//!
//! ## Тестирование
//!
//! Готовый docker-образ для тестов - [GitHub](https://github.com/cybcon/modbus-server).
//!
//! Запускается через docker compose в корне. [Инструкция](../doc/development.md).
//!
//! # Пример
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_modbus_client_simulator/main.rs")]
//! ```
//!

mod component;
mod config;
pub mod conversion;
mod error;
mod fn_process;

/// Обмен данными с устройством, поддерживающим Modbus TCP сервер.
///
/// См. [документацию](https://docs.rs/rsiot-modbus-client/latest/)
pub use component::{COMPONENT_NAME, Cmp};
pub use config::*;

type Result<T> = std::result::Result<T, error::Error>;
