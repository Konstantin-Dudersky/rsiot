use esp_idf_svc::hal::{peripheral::Peripheral, rmt::RmtChannel};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::Config;

pub async fn fn_process<TMsg, TService, TPeripheral, TRmt>(
    config: Config<TMsg, TPeripheral, TRmt>,
    mut msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TPeripheral: RmtChannel,
    TRmt: Peripheral<P = TPeripheral> + 'static,
    TService: ServiceBound,
{
    let mut ws2812 = Ws2812Esp32Rmt::new(config.rmt_channel, config.pin)?;

    while let Ok(msg) = msg_bus.recv_input().await {
        let color = (config.fn_input)(&msg);
        let Some(color) = color else { continue };
        ws2812.write_nocopy(color)?;
    }

    Err(super::Error::FnProcessEnd)
}
