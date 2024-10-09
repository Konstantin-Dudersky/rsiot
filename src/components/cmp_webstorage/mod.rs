//! Хранение и загрузка сообщений используя LocalStorage или SessionStorage браузера.
//!
//! Подробнее на [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API).
//!
//! Используется модуль `storage` библиотеки [gloo](https://docs.rs/gloo/latest/gloo/storage/index.html).

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

#[cfg(test)]
mod test;

pub use component::Cmp;
pub use config::{Config, ConfigStorageKind};
pub use error::Error;

type Result<T> = std::result::Result<T, error::Error>;
