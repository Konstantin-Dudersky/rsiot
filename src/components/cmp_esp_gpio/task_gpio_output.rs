use esp_idf_svc::hal::gpio::PinDriver;

use crate::{executor::MsgBusInput, message::MsgDataBound};

use super::{ConfigGpioOutput, Error};

pub struct GpioOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub config_output: ConfigGpioOutput<TMsg>,
}

impl<TMsg> GpioOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut pin =
            PinDriver::output(self.config_output.peripherals).map_err(Error::CreatePinDriver)?;

        // Значение по-умолчанию
        if self.config_output.default {
            pin.set_high().map_err(Error::SetGpioOutput)?;
        } else {
            pin.set_low().map_err(Error::SetGpioOutput)?;
        }

        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let level = (self.config_output.fn_input)(msg);
            let Some(level) = level else { continue };
            if level {
                pin.set_high().map_err(Error::SetGpioOutput)?;
            } else {
                pin.set_low().map_err(Error::SetGpioOutput)?;
            }
        }

        Err(Error::TaskEndGpioOutput)
    }
}
