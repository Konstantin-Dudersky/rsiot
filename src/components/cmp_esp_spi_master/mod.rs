//! Компонент для работы с подчиненными устройствами по шине SPI

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::Cmp;
pub use config::{Config, ConfigDevice};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

enum InnerMessage<TMsg> {
    Message(crate::message::Message<TMsg>),
    Periodic,
}
