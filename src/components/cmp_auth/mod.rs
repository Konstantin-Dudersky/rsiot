//! Компонент авторизации пользователей
//!
//! TODO - переделать, поскольку удалил MsgTrace

mod component;
mod config;
mod error;
mod fn_process;
mod token_payload;

#[cfg(test)]
mod test;

pub use component::Cmp;
pub use config::{Config, ConfigStore, ConfigStoreLocalItem};
pub use error::Error;

type Result<TMsg> = std::result::Result<TMsg, Error>;
