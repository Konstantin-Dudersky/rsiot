use async_trait::async_trait;
use esp_idf_svc::hal::{peripheral::Peripheral, rmt::RmtChannel};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

pub const COMPONENT_NAME: &str = "cmp_esp_led";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TPeripheral, TRmt> IComponentProcess<Config<TMsg, TPeripheral, TRmt>, TMsg>
    for Component<Config<TMsg, TPeripheral, TRmt>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TRmt: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: RmtChannel,
{
    async fn process(
        &self,
        config: Config<TMsg, TPeripheral, TRmt>,
        msg_bus: CmpInOut<TMsg>,
    ) -> CmpResult {
        let input = msg_bus.msgbus_input(COMPONENT_NAME);
        fn_process(config, input).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_led
pub type Cmp<TMsg, TPeripheral, TRmt> = Component<Config<TMsg, TPeripheral, TRmt>, TMsg>;
