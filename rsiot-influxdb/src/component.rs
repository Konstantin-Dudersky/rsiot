use async_trait::async_trait;

use rsiot_component_core::{
    Cache, CmpOutput, Component, ComponentError, ComponentInput, IComponentProcess,
};
use rsiot_messages_core::IMessage;
use tracing::error;

use crate::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        input: ComponentInput<TMsg>,
        output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        error!("Influxdb client component start execution");
        fn_process(input, output, config).await?;
        error!("Influxdb client component end execution");
        Ok(())
    }
}

pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
