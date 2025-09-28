use async_trait::async_trait;
use esp_idf_svc::hal::{
    peripheral::Peripheral,
    spi::{Spi, SpiAnyPins},
};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_esp_spi_master";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TSpi, TPeripheral> IComponentProcess<Config<TMsg, TSpi, TPeripheral>, TMsg>
    for Component<Config<TMsg, TSpi, TPeripheral>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TSpi, TPeripheral>,
        msg_bus: CmpInOut<TMsg>,
    ) -> CmpResult {
        let (input, output) = msg_bus.msgbus_input_output(COMPONENT_NAME);
        fn_process(config, input, output).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_spi_master
pub type Cmp<TMsg, TSpi, TPeripheral, const MESSAGE_LEN: usize> =
    Component<Config<TMsg, TSpi, TPeripheral>, TMsg>;
