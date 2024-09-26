mod input;
mod output;

pub use input::Input;
pub use output::Output;

use super::{Error, Result};

type TaskInput<TMsg> = tokio::sync::mpsc::Receiver<crate::message::Message<TMsg>>;
type TaskOutput<TMsg> = tokio::sync::mpsc::Sender<crate::message::Message<TMsg>>;
