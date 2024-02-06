//! Компонент для сохранения / извлечения данных из сохраняемой области памяти (NVS - Non-Volatile
//! Storage)

mod component;
mod config;
mod error;
mod fn_process;

pub mod cmp_storage_esp {
    pub use super::component::Cmp;
    pub use super::config::Config;
}
