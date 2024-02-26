use std::time::Duration;

use async_trait::async_trait;
use esp_idf_svc::hal::peripherals::Peripherals;
use tokio::time::sleep;
use tracing::info;

use rsiot::component_core::{
    Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot::message::MsgDataBound;

pub struct Config {
    pub per: &'static Peripherals,
}

#[async_trait(?Send)]
impl<TMsg> IComponentProcess<Config, TMsg> for Component<Config, TMsg>
where
    TMsg: MsgDataBound,
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
