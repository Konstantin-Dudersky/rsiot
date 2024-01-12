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
//! # Диаграмма
//!
//! ![](./doc/component-modbus-client.svg)
#![doc = include_str!("../doc/component-modbus-client.svg")]
//!
//! # Пример
//!
//! ```rust
#![doc = include_str!("../examples/modbus_tcp_client.rs")]
//! ```
//!
//! TODO - рестарт не работает

mod component;
mod config;
pub mod conversion;
mod error;
mod fn_process;

/// Обмен данными с устройством, поддерживающим Modbus TCP сервер.
///
/// См. [документацию](https://docs.rs/rsiot-modbus-client/latest/)
pub mod cmp_modbus_client {
    pub use crate::component::Cmp;
    pub use crate::config::*;
    pub use crate::conversion;
}

type Result<T, TMessage> = std::result::Result<T, error::Error<TMessage>>;
