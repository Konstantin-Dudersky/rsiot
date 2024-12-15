use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage, TService> IComponentProcess<Config<TMessage>, TMessage, TService>
    for Component<Config<TMessage>, TMessage, TService>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMessage>,
        input: CmpInOut<TMessage, TService>,
    ) -> Result<(), ComponentError> {
        fn_process(
            input.clone_with_new_id("cmp_websocket_client", AuthPermissions::FullAccess),
            config,
        )
        .await?;
        Ok(())
    }
}

/// Компонент cmp_websocket_client
pub type Cmp<TMessage, TService> = Component<Config<TMessage>, TMessage, TService>;
