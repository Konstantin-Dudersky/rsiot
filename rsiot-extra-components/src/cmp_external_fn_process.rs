use async_trait::async_trait;
use futures::future::{BoxFuture, LocalBoxFuture};
use tracing::info;

use rsiot_component_core::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, ComponentResult,
    IComponentProcess,
};
use rsiot_messages_core::IMessage;

#[cfg(feature = "single-thread")]
pub struct Config {
    pub fn_process: Box<dyn Fn() -> LocalBoxFuture<'static, ComponentResult>>,
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMessage> IComponentProcess<Config, TMessage> for Component<Config, TMessage>
where
    TMessage: IMessage,
{
    async fn process(
        &self,
        config: Config,
        _input: ComponentInput<TMessage>,
        _output: ComponentOutput<TMessage>,
        _cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        info!("Start component cmp_extrenal_fn_process");
        // let res = (config.fn_process)().await;
        Ok(())
    }
}

#[cfg(feature = "single-thread")]
pub type Cmp<TMessage> = Component<Config, TMessage>;
