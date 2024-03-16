//! Компонент для сохранения / извлечения данных из сохраняемой области памяти (NVS - Non-Volatile
//! Storage)

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::Config;

// TODO - переименовать в cmp_eps_nvs ?
