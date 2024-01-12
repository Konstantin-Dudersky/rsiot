//! Настройки логгирования для разных платформ.
//!
//! Для настройки логгирования нужно задать переменную `RUST_LOG`.
//!
//! TODO - Примеры задания переменной `RUST_LOG`
//!

mod error;
pub use error::Error;

#[cfg(target_arch = "wasm32")]
mod target_wasm32;
#[cfg(target_arch = "wasm32")]
pub use target_wasm32::configure_logging;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod target_x86_64;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub use target_x86_64::configure_logging;
