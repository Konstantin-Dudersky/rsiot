mod input;
mod output;
mod uart_comm;

pub use input::Input;
pub use output::Output;
pub use uart_comm::UartComm;

use super::{Buffer, Error, Result};

type TaskOutput<T> = tokio::sync::mpsc::Sender<T>;
