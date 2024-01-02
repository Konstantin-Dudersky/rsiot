//! Компонент для сохранения / извлечения данных из сохраняемой области памяти (NVS - Non-Volatile
//! Storage)

mod config;
mod error;
mod fn_process;
mod new;

pub mod cmp_storage_esp {
    pub use super::config::Config;
    pub use super::new::new;
}
