use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;
use tracing::info;

use rsiot::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

pub struct Config {}

#[async_trait]
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
