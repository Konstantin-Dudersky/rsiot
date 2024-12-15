use async_trait::async_trait;
use esp_idf_svc::hal::{peripheral::Peripheral, rmt::RmtChannel};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TService, TPeripheral, TRmt>
    IComponentProcess<Config<TMsg, TPeripheral, TRmt>, TMsg, TService>
    for Component<Config<TMsg, TPeripheral, TRmt>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TRmt: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: RmtChannel,
    TService: ServiceBound,
{
    async fn process(
        &self,
        config: Config<TMsg, TPeripheral, TRmt>,
        msg_bus: CmpInOut<TMsg, TService>,
    ) -> CmpResult {
        let in_out = msg_bus.clone_with_new_id("cmp_esp_led", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_led
pub type Cmp<TMsg, TService, TPeripheral, TRmt> =
    Component<Config<TMsg, TPeripheral, TRmt>, TMsg, TService>;
