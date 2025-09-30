use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use tracing::trace;

use crate::{executor::MsgBusInput, message::MsgDataBound};

use super::{ConfigGpioOutput, Error};

pub struct GpioOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_input: MsgBusInput<TMsg>,

    pub config: ConfigGpioOutput<TMsg>,
}

impl<TMsg> GpioOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut chip = Chip::new(self.config.dev_gpio).map_err(Error::GpioSetup)?;

        let line = chip
            .get_line(self.config.gpio_line as u32)
            .map_err(Error::GpioSetup)?;

        let default_state = if self.config.default_state { 1 } else { 0 };

        let output_handle = line
            .request(
                LineRequestFlags::OUTPUT,
                default_state,
                self.config.description,
            )
            .map_err(Error::GpioSetup)?;

        while let Ok(msg) = self.msgbus_input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let value = (self.config.fn_gpio_output)(msg);
            let Some(value) = value else {
                continue;
            };
            let value = if value { 1 } else { 0 };
            trace!(
                "Setting GPIO output {}-{} to {}",
                self.config.dev_gpio, self.config.gpio_line, value
            );
            output_handle
                .set_value(value)
                .map_err(Error::GpioSetValue)?;
        }

        Err(Error::TaskGpioOutputEnd)
    }
}
