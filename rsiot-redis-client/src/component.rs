use async_trait::async_trait;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::{IMessageChannel, MsgDataBound};

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
        mut input: CmpInput<TMessage>,
        mut output: CmpOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        cmp_set_component_id(&mut input, &mut output, "cmp_redis_client");
        fn_process(input, output, config, cache).await
    }
}

pub type Cmp<TMessage, TMessageChannel> =
    Component<ConfigAlias<TMessage, TMessageChannel>, TMessage>;
