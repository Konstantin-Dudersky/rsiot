#![doc = include_str!("../README.md")]
//!
//! # Пример
//!
//! ```rust
#![doc = include_str!("../examples/ex1.rs")]
//! ```

mod cli;
mod create_env_file;
mod errors;
mod iconfig;
mod load_config;

pub use cli::env_vars_cli;
pub use errors::Errors;
pub use iconfig::IConfig;
pub use load_config::load_config;
