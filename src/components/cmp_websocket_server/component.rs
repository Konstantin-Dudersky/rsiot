use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage, TService> IComponentProcess<ConfigAlias<TMessage>, TMessage, TService>
    for Component<ConfigAlias<TMessage>, TMessage, TService>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage>,
        input: CmpInOut<TMessage, TService>,
    ) -> Result<(), ComponentError> {
        fn_process(
            input.clone_with_new_id("cmp_websocket_server", AuthPermissions::FullAccess),
            config.0,
        )
        .await
    }
}

/// Компонент cmp_websocker_server
pub type Cmp<TMessage, TService> = Component<ConfigAlias<TMessage>, TMessage, TService>;
