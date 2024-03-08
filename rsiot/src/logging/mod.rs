#![allow(clippy::needless_doctest_main)]
//! Настройки логгирования для разных платформ.
//!
//! Для настройки логгирования нужно задать переменную `RUST_LOG`.
//!
//! ## Способы задания RUST_LOG
//!
//! ### Запуск в контейнере
//!
//! В файле `docker-compose.yaml` для сервиса указать:
//!
//! ```yaml
//! services:
//!   rust_service:
//!     environment:
//!       - RUST_LOG=info
//! ```
//!
//! Значение переменной можно задавать для каждого сервиса оданиково.
//!
//! ### Запуск в контейнере, сохранение в файле `.env`
//!
//! В файле `docker-compose.yaml` для сервиса указать:
//!
//! ```yaml
//! services:
//!   rust_service:
//!     env_file: .env
//! ```
//!
//! Значение переменной будет одинаково для всех сервисов
//!
//! ### Задание в compile-time
//!
//! Платформы WASM, ESP не могут считывать переменные окружения, поэтому значение необходимо
//! прописывать на этапе компиляции.
//!
//! Чтобы значение переменной считывалось из файла:
//!
//! - создать файл .env в корне проекта
//! - прописать в файле переменную в виде `RUST_LOG = info`
//! - если изменить только переменную, без изменения кода, то перекомпиляции не будет. Поэтому можно
//!   создать файл `build.rs` в корне проекта с содержимым:
//!
//! ```rust
//! pub fn main() {
//!     println!("cargo:rerun-if-changed=.env");
//! }
//! ```
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
