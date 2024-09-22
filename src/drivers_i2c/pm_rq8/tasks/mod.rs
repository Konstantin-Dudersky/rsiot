mod i2c_comm;
mod input;
mod input_periodic;
mod output;

pub use i2c_comm::I2cComm;
pub use input::Input;
pub use input_periodic::InputPeriodic;
pub use output::Output;

use super::{Error, I2cRequest, I2cResponse, Result};

type TaskInput<T> = tokio::sync::mpsc::Receiver<T>;
type TaskOutput<T> = tokio::sync::mpsc::Sender<T>;
