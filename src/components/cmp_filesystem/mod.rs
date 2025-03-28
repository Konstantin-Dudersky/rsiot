//! Компонент для сохранения данных в файловой системе и загрузки данных из файловой системы
//!
//! # Пример
//!
//! ## Файл `config_filesystem/mod.rs`
//! ```rust
#![doc = include_str!("../../../examples/cmp_filesystem/config_filesystem.rs")]
//! ```

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{BufferBound, CallFnOutputKind, Config};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
