use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;
use tracing::info;

use rsiot_component_core::{CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::MsgDataBound;

pub struct Config {}

#[async_trait(?Send)]
impl<TMsg> IComponentProcess<Config, TMsg> for Component<Config, TMsg>
where
    TMsg: MsgDataBound,
{
    async fn process(&self, _config: Config, _input: CmpInOut<TMsg>) -> Result<(), ComponentError> {
        loop {
            info!("Component 2");
            sleep(Duration::from_secs(2)).await;
        }
    }
}

pub type Cmp<TMsg> = Component<Config, TMsg>;
