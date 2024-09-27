//! Хранение и загрузка сообщений используя LocalStorage или SessionStorage браузера. Подробнее на #link("https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API")[MDN].
//!
//! Используется модуль `storage` библиотеки #link("https://docs.rs/gloo/latest/gloo/storage/index.html")[gloo].

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::Cmp;
pub use config::{Config, ConfigStorageKind};
pub use error::Error;

type Result<T> = std::result::Result<T, error::Error>;
