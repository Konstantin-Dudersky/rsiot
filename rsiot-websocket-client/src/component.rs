use async_trait::async_trait;

use rsiot_component_core::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMessage> IComponentProcess<ConfigAlias<TMessage>, TMessage>
    for Component<ConfigAlias<TMessage>, TMessage>
where
    TMessage: IMessage + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage>,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        _cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(input, output, config).await
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMessage> IComponentProcess<ConfigAlias<TMessage>, TMessage>
    for Component<ConfigAlias<TMessage>, TMessage>
where
    TMessage: IMessage + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage>,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        _cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(input, output, config).await
    }
}

pub type Cmp<TMessage> = Component<ConfigAlias<TMessage>, TMessage>;
