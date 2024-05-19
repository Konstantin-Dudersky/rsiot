use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage> IComponentProcess<Config<TMessage>, TMessage>
    for Component<Config<TMessage>, TMessage>
where
    TMessage: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMessage>,
        input: CmpInOut<TMessage>,
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
pub type Cmp<TMessage> = Component<Config<TMessage>, TMessage>;
