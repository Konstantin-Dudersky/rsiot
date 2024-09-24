//! Отправка сообщений в телеграм

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;
mod telegram_bot;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
use telegram_bot::TelegramBot;
