mod component;
mod config;
mod derive_item;
mod derive_item_process;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::Config;
pub use derive_item::DeriveItem;
pub use derive_item_process::DeriveItemProcess;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
