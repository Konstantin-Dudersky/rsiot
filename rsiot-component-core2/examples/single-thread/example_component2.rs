use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;
use tracing::info;

use rsiot_component_core2::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

pub struct Config2 {}

#[async_trait(?Send)]
impl<TMessage> IComponentProcess<Config2, TMessage> for Component<Config2, TMessage>
where
    TMessage: IMessage,
{
    async fn process(
        &self,
        _config: Config2,
        _input: ComponentInput<TMessage>,
        _output: ComponentOutput<TMessage>,
        _cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        loop {
            info!("Component 2");
            sleep(Duration::from_secs(2)).await;
        }
    }
}

pub type Component2<TMessage> = Component<Config2, TMessage>;
