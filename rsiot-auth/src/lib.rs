mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigStoreKind};
pub use error::Error;

type Result<TMsg> = std::result::Result<TMsg, Error>;
