use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TService, const MESSAGE_LEN: usize>
    IComponentProcess<Config<TMsg, MESSAGE_LEN>, TMsg, TService>
    for Component<Config<TMsg, MESSAGE_LEN>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, MESSAGE_LEN>,
        msg_bus: CmpInOut<TMsg, TService>,
    ) -> CmpResult {
        let in_out =
            msg_bus.clone_with_new_id("cmp_linux_uart_master", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_linux_uart_master
pub type Cmp<TMsg, TService, const MESSAGE_LEN: usize> =
    Component<Config<TMsg, MESSAGE_LEN>, TMsg, TService>;
