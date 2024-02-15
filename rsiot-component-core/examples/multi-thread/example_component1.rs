use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;
use tracing::info;

use rsiot_component_core::{
    Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::IMessage;

pub struct Config {}

#[async_trait]
impl<TMsg> IComponentProcess<Config, TMsg> for Component<Config, TMsg>
where
    TMsg: IMessage,
{
    async fn process(
        &self,
        _config: Config,
        _input: CmpInput<TMsg>,
        _output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        loop {
            info!("Component 1");
            sleep(Duration::from_secs(2)).await;
        }
    }
}

pub type Cmp<TMsg> = Component<Config, TMsg>;
