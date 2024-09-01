use super::{Error, Result};

mod i2c_comm;
mod input;
mod output;

pub use i2c_comm::I2cComm;
pub use input::Input;
pub use output::Output;
