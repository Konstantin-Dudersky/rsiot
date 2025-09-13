use linux_embedded_hal::gpio_cdev::{AsyncLineEventHandle, Chip};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{ConfigWrite, Error};

pub struct Write<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msg_bus: CmpInOut<TMsg>,

    pub config: ConfigWrite<TMsg>,
}

impl<TMsg> Write<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), Error> {
        let mut chip = Chip::new(self.config.dev_gpio).map_err(Error::GpioSetup)?;
        let line = chip
            .get_line(self.config.gpio_line as u32)
            .map_err(Error::GpioSetup)?;

        while let Ok(msg) = self.msg_bus.recv_input().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
        }

        let mut events = AsyncLineEventHandle::new(line.events(
            LineRequestFlags::INPUT,
            EventRequestFlags::BOTH_EDGES,
            "gpioevents",
        )?)?;

        while let Some(event) = events.next().await {
            let event = event?;
            println!("GPIO Event: {:?}", event);
        }
    }
}
