//! Коммуникация с модулем PM-DI16

mod config;
mod device;
mod error;
mod tasks;

pub use config::Config;
pub use device::Device;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
