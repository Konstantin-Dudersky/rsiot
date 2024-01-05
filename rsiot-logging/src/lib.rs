//! Настройки логгирования для разных платформ.
//!
//! Для настройки логгирования нужно задать переменную `RUST_LOG`.
//!
//! Предусмотреть разные способы логгирования:
//! - для PC (linux, windows, ...) - loki, stdout
//! - для WASM - в console (как настроить уровень через переменную?)
//! - для ESP - в stdout (как настроить уровень)
//!
//!
//! TODO - Примеры задания переменной `RUST_LOG`
//! TODO - настройка для ESP

#![allow(unused_imports)]

mod error;
mod target_wasm32;
mod target_x86_64;

pub use error::Error;

#[cfg(target_arch = "wasm32")]
pub use target_wasm32::configure_logging;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub use target_x86_64::configure_logging;
