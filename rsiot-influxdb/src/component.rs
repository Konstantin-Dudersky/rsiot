use async_trait::async_trait;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::MsgDataBound;
use tracing::{error, info};

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<ConfigAlias<TMsg>, TMsg> for Component<ConfigAlias<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMsg>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_id(&mut input, &mut output, "cmp_influxdb");
        info!("Influxdb client component start execution");
        fn_process(input, output, config.0).await?;
        error!("Influxdb client component end execution");
        Ok(())
    }
}

pub type Cmp<TMsg> = Component<ConfigAlias<TMsg>, TMsg>;
