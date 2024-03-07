mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result = std::result::Result<(), Error>;
