use async_trait::async_trait;

use rsiot_component_core::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMsg> IComponentProcess<ConfigAlias, TMsg> for Component<ConfigAlias, TMsg>
where
    TMsg: IMessage,
{
    async fn process(
        &self,
        config: ConfigAlias,
        input: ComponentInput<TMsg>,
        _output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(input, config).await
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg> IComponentProcess<ConfigAlias, TMsg> for Component<ConfigAlias, TMsg>
where
    TMsg: IMessage,
{
    async fn process(
        &self,
        config: ConfigAlias,
        input: ComponentInput<TMsg>,
        _output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(input, config).await
    }
}

pub type Cmp<TMsg> = Component<ConfigAlias, TMsg>;
