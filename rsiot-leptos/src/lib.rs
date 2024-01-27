#![cfg(target_arch = "wasm32")]

mod cmp_leptos;
mod error;
mod global_state;
pub mod msg_to_signals;

pub use cmp_leptos::{Cmp, Config};
pub use error::Error;
pub use global_state::GlobalState;

type Result<TMsg> = std::result::Result<(), Error<TMsg>>;
