use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;
use tracing::info;

use rsiot_component_core2::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

pub struct Config1 {}

#[async_trait]
impl<TMessage> IComponentProcess<Config1, TMessage> for Component<Config1, TMessage>
where
    TMessage: IMessage,
{
    async fn process(
        &self,
        _config: Config1,
        _input: ComponentInput<TMessage>,
        _output: ComponentOutput<TMessage>,
        _cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        loop {
            info!("Component 1");
            sleep(Duration::from_secs(2)).await;
        }
    }
}

pub type Component1<TMessage> = Component<Config1, TMessage>;
