//! Компонент для работы с GPIO Raspberry Pi
//!
//! Обертка над библиотекой [rppal](https://crates.io/crates/rppal)
//!
//! Запускать программу скорее всего нужно из-под sudo.
//!
//! Распиновка - https://www.raspberrypi.com/documentation/computers/os.html#gpio-and-the-40-pin-header
//!
//!

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigInput, ConfigOutput};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
