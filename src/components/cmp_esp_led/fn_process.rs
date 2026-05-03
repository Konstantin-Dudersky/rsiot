use esp_idf_svc::hal::rmt::RmtChannel;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

use crate::{executor::MsgBusInput, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg, TRmt>(
    config: Config<TMsg, TRmt>,
    mut input: MsgBusInput<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TRmt: RmtChannel + 'static,
{
    let mut ws2812 = Ws2812Esp32Rmt::new(config.rmt_channel, config.pin)?;

    while let Ok(msg) = input.recv().await {
        let config = (config.fn_input)(&msg);
        let Some(config) = config else { continue };

        let mut leds = vec![];
        for (amount, color) in config {
            for _ in 0..amount {
                leds.push(color);
            }
        }

        ws2812.write_nocopy(leds)?;
    }

    Err(super::Error::FnProcessEnd)
}
