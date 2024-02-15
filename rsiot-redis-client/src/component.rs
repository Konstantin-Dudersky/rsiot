use async_trait::async_trait;

use rsiot_component_core::{
    Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::{IMessage, IMessageChannel};

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMessage, TMessageChannel> IComponentProcess<ConfigAlias<TMessage, TMessageChannel>, TMessage>
    for Component<ConfigAlias<TMessage, TMessageChannel>, TMessage>
where
    TMessage: IMessage + 'static,
    TMessageChannel: IMessageChannel + 'static,
    Self: Sync,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage, TMessageChannel>,
        input: CmpInput<TMessage>,
        output: CmpOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(input, output, config, cache).await
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMessage, TMessageChannel> IComponentProcess<ConfigAlias<TMessage, TMessageChannel>, TMessage>
    for Component<ConfigAlias<TMessage, TMessageChannel>, TMessage>
where
    TMessage: IMessage + 'static,
    TMessageChannel: IMessageChannel + 'static,
    Self: Sync,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage, TMessageChannel>,
        input: CmpInput<TMessage>,
        output: CmpOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(input, output, config, cache).await
    }
}

pub type Cmp<TMessage, TMessageChannel> =
    Component<ConfigAlias<TMessage, TMessageChannel>, TMessage>;
