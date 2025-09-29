use async_trait::async_trait;

use tracing::error;

use crate::{
    components::cmp_logger::COMPONENT_NAME,
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::{Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let input = in_out.msgbus_input(COMPONENT_NAME);
        fn_process(input, config).await?;
        error!("Influxdb client component end execution");
        Ok(())
    }
}

/// Компонент cmp_influxdb
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
