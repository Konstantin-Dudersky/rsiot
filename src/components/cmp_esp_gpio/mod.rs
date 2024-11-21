//! Компонент для работы с входами и выходами GPIO микроконтроллера ESP

mod component;
mod config;
mod error;
mod fn_process;

#[cfg(test)]
mod test;

pub use component::Cmp;
pub use config::{Config, ConfigGpioInput, ConfigGpioOutput};
pub use error::Error;
pub use esp_idf_svc::hal::gpio::Pull;

type Result<T> = std::result::Result<T, Error>;
