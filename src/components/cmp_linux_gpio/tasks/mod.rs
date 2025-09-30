mod gpio_input;
mod gpio_output;

use super::{ConfigGpioInput, ConfigGpioOutput, Error};

pub use {gpio_input::GpioInput, gpio_output::GpioOutput};
