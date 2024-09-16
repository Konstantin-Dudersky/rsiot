mod input;
mod period;
mod spi_comm;

pub use input::Input;
pub use period::Period;
pub use spi_comm::SpiComm;

use super::{Config, InnerMessage, Result};
