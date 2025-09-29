use async_trait::async_trait;
use esp_idf_svc::hal::{peripheral::Peripheral, uart::Uart};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_esp_uart_slave";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TUart, TPeripheral, TBufferData>
    IComponentProcess<Config<TMsg, TUart, TPeripheral, TBufferData>, TMsg>
    for Component<Config<TMsg, TUart, TPeripheral, TBufferData>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
    TBufferData: 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TUart, TPeripheral, TBufferData>,
        msgbus_linker: CmpInOut<TMsg>,
    ) -> CmpResult {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_uart_slave
pub type Cmp<TMsg, TUart, TPeripheral, TBufferData> =
    Component<Config<TMsg, TUart, TPeripheral, TBufferData>, TMsg>;
