use async_trait::async_trait;
use esp_idf_svc::hal::{
    peripheral::Peripheral,
    spi::{Spi, SpiAnyPins},
};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TService, TSpi, TPeripheral>
    IComponentProcess<Config<TMsg, TSpi, TPeripheral>, TMsg, TService>
    for Component<Config<TMsg, TSpi, TPeripheral>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TSpi, TPeripheral>,
        msg_bus: CmpInOut<TMsg, TService>,
    ) -> CmpResult {
        let in_out = msg_bus.clone_with_new_id("cmp_esp_spi_master", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_spi_master
pub type Cmp<TMsg, TService, TSpi, TPeripheral> =
    Component<Config<TMsg, TSpi, TPeripheral>, TMsg, TService>;
