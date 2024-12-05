use async_trait::async_trait;
use esp_idf_svc::hal::{peripheral::Peripheral, uart::Uart};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process, RequestResponseBound};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData, const MESSAGE_LEN: usize>
    IComponentProcess<
        Config<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData, MESSAGE_LEN>,
        TMsg,
    >
    for Component<
        Config<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData, MESSAGE_LEN>,
        TMsg,
    >
where
    TMsg: MsgDataBound + 'static,
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
    TRequest: 'static + RequestResponseBound,
    TResponse: 'static + RequestResponseBound,
    TBufferData: 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData, MESSAGE_LEN>,
        msg_bus: CmpInOut<TMsg>,
    ) -> CmpResult {
        let in_out = msg_bus.clone_with_new_id("cmp_esp_uart_slave", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_uart_slave
pub type Cmp<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData, const MESSAGE_LEN: usize> =
    Component<Config<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData, MESSAGE_LEN>, TMsg>;
