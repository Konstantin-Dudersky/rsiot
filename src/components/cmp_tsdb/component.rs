use async_trait::async_trait;

use crate::{
    executor::{Component, ComponentError, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{Error, Row, config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_timescaledb";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TFnInput> IComponentProcess<Config<TMsg, TFnInput>, TMsg>
    for Component<Config<TMsg, TFnInput>, TMsg>
where
    TMsg: 'static + MsgDataBound,
    TFnInput: 'static + Fn(&TMsg) -> Result<Option<Vec<Row>>, Error> + Send + Sync,
{
    async fn process(
        &self,
        config: Config<TMsg, TFnInput>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(msgbus_linker.init(COMPONENT_NAME), config).await?;
        Ok(())
    }
}

/// Компонент cmp_timescaledb
pub type Cmp<TMsg, TFnInput> = Component<Config<TMsg, TFnInput>, TMsg>;
