//! Компонент для предоставления данных сторонним системам по протоколу HTTP.
//!
//! Обрабатывает GET и PUT запросы.
//!
//! Компонент основан на веб-фреймворке [Axum](https://crates.io/crates/axum).
//!
//! # Структура
//!
#![doc = include_str!("doc/diagram.svg")]
//!
//! | Название | Описание |
//! |----------|----------|
//! | UpdateGetEndpoints | Задача получает сообщения из шины MsgBus и обновляет данные в структуре общего состояния SharedState на основе функции GetEndpointConfig::fn_input. |
//! | AxumServe | Задача обеспечивает запуск и работу веб-фреймворка Axum. При обработке GET-запросов возвращает данные из общего состояния Shared State. При обработке PUT-запросов формирует исходящие сообщения на основе функции PutEndpointConfig::fn_output. |
//!
//! Данные возвращаются в виде структур Json. Для более удобного просмотра можно использовать
//! расширения к браузеру, например [JSON Beautifier & Editor](https://chromewebstore.google.com/detail/json-beautifier-editor/lpopeocbeepakdnipejhlpcmifheolpl)
//!
//! # Примеры
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_http_server_and_client/cmp_http_server.rs")]
//! ```
//!
//! # Тестирование
//!
//! См. папку .bruno
//!

mod component;
mod config;
mod error;
mod fn_process;
mod routes;
mod shared_state;
mod tasks;

pub use crate::components_config::http_server::{
    GetEndpoint, GetEndpointConfig, GetEndpointsCollection, PutEndpoint, PutEndpointConfig,
    PutEndpointsCollection,
};
pub use component::{COMPONENT_NAME, Cmp};
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
