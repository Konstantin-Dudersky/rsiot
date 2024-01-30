use async_trait::async_trait;

use rsiot_component_core::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use crate::fn_process::fn_process;

#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMsg> IComponentProcess<crate::Config<TMsg>, TMsg> for Component<crate::Config<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: crate::Config<TMsg>,
        input: ComponentInput<TMsg>,
        output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(input.resubscribe(), output.clone(), config.clone()).await?;
        Ok(())
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg> IComponentProcess<crate::Config<TMsg>, TMsg> for Component<crate::Config<TMsg>, TMsg>
where
    TMsg: IMessage,
{
    async fn process(
        &self,
        _config: crate::Config<TMsg>,
        _input: ComponentInput<TMsg>,
        _output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        todo!()
    }
}

pub type Cmp<TMsg> = Component<crate::Config<TMsg>, TMsg>;
