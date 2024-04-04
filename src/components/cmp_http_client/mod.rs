//! Компонент HTTP-клиент

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::*;
pub use error::Error;

type Result<T, TMessage> = std::result::Result<T, error::Error<TMessage>>;
