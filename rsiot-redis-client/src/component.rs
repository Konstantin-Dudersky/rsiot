use async_trait::async_trait;

use rsiot_component_core::{CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::{AuthPermissions, IMessageChannel, MsgDataBound};

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage, TMessageChannel> IComponentProcess<ConfigAlias<TMessage, TMessageChannel>, TMessage>
    for Component<ConfigAlias<TMessage, TMessageChannel>, TMessage>
where
    TMessage: MsgDataBound + 'static,
    TMessageChannel: IMessageChannel + 'static,
    Self: Sync,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage, TMessageChannel>,
        input: CmpInOut<TMessage>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(
            input.clone_with_new_id("cmp_redis_client", AuthPermissions::FullAccess),
            config,
        )
        .await
    }
}

pub type Cmp<TMessage, TMessageChannel> =
    Component<ConfigAlias<TMessage, TMessageChannel>, TMessage>;
