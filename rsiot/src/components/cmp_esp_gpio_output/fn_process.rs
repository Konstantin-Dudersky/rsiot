use esp_idf_svc::hal::gpio::OutputPin;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TPin, TMsg>(
    mut config: Config<TPin, TMsg>,
    mut in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TPin: OutputPin,
{
    while let Ok(msg) = in_out.recv_input().await {
        let level = (config.fn_input)(msg);
        let Some(level) = level else { continue };
        if config.is_low_triggered ^ level {
            config.driver.set_low().unwrap();
        } else {
            config.driver.set_high().unwrap();
        }
    }
    Ok(())
}
