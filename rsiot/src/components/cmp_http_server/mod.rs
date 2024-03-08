//! Компонент для получения данных через HTTP server.
//!
//! Ссылки:
//!
//! - [Документация docs.rs](https://docs.rs/rsiot-http-server/latest/)
//!
//! - [Репозиторий GitHub](https://github.com/Konstantin-Dudersky/rsiot/tree/main/rsiot-http-server)
//!
//! - [Примеры](https://github.com/Konstantin-Dudersky/rsiot/tree/main/rsiot-http-server/examples)
//!
#![doc = include_str!("../../../doc/api_description.md")]
//!
//! ## Пример
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_http_server/cmp_http_server.rs")]
//! ```
//!
//! ## Тестирование
//!
//! См. папку .bruno
//!

#![cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]

mod component;
mod config;
mod error;
mod fn_process;
mod routes;
mod shared_state;

pub use component::Cmp;
pub use config::Config;
