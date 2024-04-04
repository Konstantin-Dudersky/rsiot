use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage> IComponentProcess<ConfigAlias<TMessage>, TMessage>
    for Component<ConfigAlias<TMessage>, TMessage>
where
    TMessage: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage>,
        input: CmpInOut<TMessage>,
    ) -> Result<(), ComponentError> {
        fn_process(
            input.clone_with_new_id("cmp_websocket_server", AuthPermissions::FullAccess),
            config.0,
        )
        .await
    }
}

/// Компонент cmp_websocker_server
pub type Cmp<TMessage> = Component<ConfigAlias<TMessage>, TMessage>;
