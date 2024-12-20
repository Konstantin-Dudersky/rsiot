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
//! Данные возвращаются в виде структур Json. Для более удобного просмотра можно использовать расширения к браузеру, например [JSON Beautifier & Editor](https://chromewebstore.google.com/detail/json-beautifier-editor/lpopeocbeepakdnipejhlpcmifheolpl)
//!
//!
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

mod component;
mod config;
mod error;
mod fn_process;
mod routes;
mod shared_state;
mod tasks;

pub use component::Cmp;
pub use config::{Config, ConfigCmpPlcData};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
