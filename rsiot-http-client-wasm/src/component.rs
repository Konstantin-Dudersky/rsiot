use async_trait::async_trait;

use rsiot_component_core::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMsg> IComponentProcess<ConfigAlias<TMsg>, TMsg> for Component<ConfigAlias<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMsg>,
        input: ComponentInput<TMsg>,
        output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        unimplemented!()
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg> IComponentProcess<ConfigAlias<TMsg>, TMsg> for Component<ConfigAlias<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMsg>,
        input: ComponentInput<TMsg>,
        output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(input, output, config)
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

pub type Cmp<TMsg> = Component<ConfigAlias<TMsg>, TMsg>;
