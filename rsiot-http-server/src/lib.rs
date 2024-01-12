mod component;
mod config;
mod error;
mod fn_process;
mod routes;
mod shared_state;

#[doc = include_str!("../README.md")]
///
/// # Диаграмма
///
#[doc = include_str!("../doc/component-http-server.svg")]
///
/// # Пример
///
/// ```rust
#[doc = include_str!("../examples/http-server-example.rs")]
/// ```
pub mod cmp_http_server {
    pub use crate::{component::Cmp, config::Config};
}

// TODO - добавить функцию преобразования выходных сообщений

// TODO - маршрут "/", куда вывести простой текст с информацией по маршрутам
