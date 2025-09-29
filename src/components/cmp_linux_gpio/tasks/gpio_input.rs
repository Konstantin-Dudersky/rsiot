use futures::StreamExt;
use linux_embedded_hal::gpio_cdev::{
    AsyncLineEventHandle, Chip, EventRequestFlags, EventType, LineRequestFlags,
};
use tracing::trace;

use crate::{
    executor::MsgBusOutput,
    message::{Message, MsgDataBound},
};

use super::{ConfigGpioInput, Error};

pub struct GpioInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_output: MsgBusOutput<TMsg>,

    pub config: ConfigGpioInput<TMsg>,
}

impl<TMsg> GpioInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), Error> {
        let mut chip = Chip::new(self.config.dev_gpio).map_err(Error::GpioSetup)?;

        let line = chip
            .get_line(self.config.gpio_line as u32)
            .map_err(Error::GpioSetup)?;

        // Читаем состояние пина GPIO
        let current_state = line
            .request(LineRequestFlags::INPUT, 0, self.config.description)
            .map_err(Error::GpioSetup)?
            .get_value()
            .map_err(Error::GpioSetup)?;
        self.send_msg(current_state > 0).await?;

        // Регистрируем события
        let line_event_handle = line
            .events(
                LineRequestFlags::INPUT,
                EventRequestFlags::BOTH_EDGES,
                self.config.description,
            )
            .map_err(Error::CreateLineEventHandle)?;

        // Регистрируем асинхронный обработчик событий
        let mut async_line_event_handle = AsyncLineEventHandle::new(line_event_handle)
            .map_err(Error::CreateAsyncLineEventHandle)?;

        while let Some(event) = async_line_event_handle.next().await {
            let event = event.map_err(Error::UnwrapEvent)?;
            trace!("GPIO Event: {:?}", event);

            let value = match event.event_type() {
                EventType::RisingEdge => true,
                EventType::FallingEdge => false,
            };

            self.send_msg(value).await?;
        }

        Err(Error::TaskGpioInputEnd)
    }

    async fn send_msg(&self, value: bool) -> Result<(), Error> {
        let msg = (self.config.fn_gpio_input)(value);
        let msg = Message::new_custom(msg);
        self.msgbus_output
            .send(msg)
            .await
            .map_err(|_| Error::TokioSyncMpscSend)
    }
}
