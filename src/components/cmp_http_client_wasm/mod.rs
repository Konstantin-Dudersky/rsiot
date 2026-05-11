//! Компонент для обмена данными со сторонними системами по протоколу HTTP.
//!
//! Версия для выполнения на платформе WASM в браузере. Реализован на основе крейта `gloo-net`.
//!
//! Документацию см. в компоненте `cmp_http_client`.

mod component;
mod config;
mod fn_process;
mod tasks;

pub use crate::components::shared_tasks::cmp_http_client::Error;
pub use component::Cmp;
pub use config::*;

type Result<T> = std::result::Result<T, Error>;
