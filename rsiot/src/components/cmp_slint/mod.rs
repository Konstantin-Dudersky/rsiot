//! Компонент для взаимодействия с библиотекой пользовательского интерфейса Slint

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<TMsg> = std::result::Result<TMsg, Error>;
