//! Комонент для математической обработки данных

mod algs;
mod component;
mod config;
mod error;
mod fn_process;

pub use algs::Algs;
pub use component::Cmp;
pub use config::{Config, IntMsgBound};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
