use async_trait::async_trait;

use rsiot_component_core::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use super::{fn_process::fn_process, Config};

#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        input: ComponentInput<TMsg>,
        output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(input, output, config)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        input: ComponentInput<TMsg>,
        output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(input, output, config)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
