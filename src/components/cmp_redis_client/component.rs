use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, IMessageChannel, MsgDataBound, ServiceBound},
};

use super::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage, TMessageChannel, TService>
    IComponentProcess<ConfigAlias<TMessage, TMessageChannel>, TMessage, TService>
    for Component<ConfigAlias<TMessage, TMessageChannel>, TMessage, TService>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TMessageChannel: IMessageChannel + 'static,
    Self: Sync,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage, TMessageChannel>,
        input: CmpInOut<TMessage, TService>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(
            input.clone_with_new_id("cmp_redis_client", AuthPermissions::FullAccess),
            config,
        )
        .await
    }
}

/// Компонент cmp_redis_client
pub type Cmp<TMessage, TMessageChannel, TService> =
    Component<ConfigAlias<TMessage, TMessageChannel>, TMessage, TService>;
