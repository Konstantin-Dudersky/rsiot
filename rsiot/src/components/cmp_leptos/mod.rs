mod cmp_leptos;
pub mod components;
pub mod create_signal_from_msg;
mod error;
mod global_state;
pub mod utils;

pub use cmp_leptos::{Cmp, Config};
pub use error::Error;
pub use global_state::GlobalState;
pub use rsiot_macros::create_signal_from_msg;

type Result = std::result::Result<(), Error>;

// TODO - скопировать вспомогательные файлы из UST
