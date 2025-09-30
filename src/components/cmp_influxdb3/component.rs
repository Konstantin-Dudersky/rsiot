use async_trait::async_trait;

use tracing::error;

use crate::{
    components::cmp_logger::COMPONENT_NAME,
    executor::{Component, ComponentError, IComponentProcess, MsgBusLinker},
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
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(msgbus_linker.init(COMPONENT_NAME), config).await?;
        error!("Influxdb client component end execution");
        Ok(())
    }
}

/// Компонент cmp_influxdb
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
