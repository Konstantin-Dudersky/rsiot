use esp_idf_svc::hal::gpio::{Level, PinDriver};

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::{ConfigGpioInput, Error};

pub struct GpioInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub in_out: CmpInOut<TMsg>,
    pub config_input: ConfigGpioInput<TMsg>,
}

impl<TMsg> GpioInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), Error> {
        let mut pin =
            PinDriver::input(self.config_input.peripherals).map_err(Error::CreatePinDriver)?;
        pin.set_pull(self.config_input.pull)
            .map_err(Error::SetPinPull)?;

        loop {
            let level = pin.get_level();
            let level = gpio_level_to_bool(&level);
            let msg = (self.config_input.fn_output)(level);
            let msg = Message::new_custom(msg);
            self.in_out
                .send_output(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
            pin.wait_for_any_edge()
                .await
                .map_err(Error::WaitForAnyEdge)?;
        }
    }
}

fn gpio_level_to_bool(level: &Level) -> bool {
    match level {
        Level::Low => false,
        Level::High => true,
    }
}
