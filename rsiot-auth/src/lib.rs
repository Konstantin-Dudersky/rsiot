#![cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigStore, ConfigStoreItem};
pub use error::Error;

type Result<TMsg> = std::result::Result<TMsg, Error>;
