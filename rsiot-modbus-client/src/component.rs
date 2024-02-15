use async_trait::async_trait;

use rsiot_component_core::{
    Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use crate::{config::ConfigNewType, fn_process::fn_process};

#[cfg(not(feature = "single-thread"))]
#[async_trait()]
impl<TMessage> IComponentProcess<ConfigNewType<TMessage>, TMessage>
    for Component<ConfigNewType<TMessage>, TMessage>
where
    TMessage: IMessage + 'static,
{
    async fn process(
        &self,
        config: ConfigNewType<TMessage>,
        input: CmpInput<TMessage>,
        output: CmpOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        fn_process(input, output, config.0, cache).await
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMessage> IComponentProcess<ConfigNewType<TMessage>, TMessage>
    for Component<ConfigNewType<TMessage>, TMessage>
where
    TMessage: IMessage + 'static,
{
    async fn process(
        &self,
        config: ConfigNewType<TMessage>,
        input: CmpInput<TMessage>,
        output: CmpOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        fn_process(input, output, config.0, cache).await
    }
}

pub type Cmp<TMessage> = Component<ConfigNewType<TMessage>, TMessage>;
