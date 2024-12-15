use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;
use tracing::info;

use rsiot::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{MsgDataBound, ServiceBound},
};

pub struct Config {}

#[async_trait]
impl<TMsg, TService> IComponentProcess<Config, TMsg, TService> for Component<Config, TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    async fn process(
        &self,
        _config: Config,
        _input: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        loop {
            info!("Component 1");
            sleep(Duration::from_secs(2)).await;
        }
    }
}

pub type Cmp<TMsg, TService> = Component<Config, TMsg, TService>;
