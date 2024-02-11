#![cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, InputConfig};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
