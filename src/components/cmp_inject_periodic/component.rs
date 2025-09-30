use async_trait::async_trait;

use crate::{
    executor::{Component, ComponentError, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_inject_periodic";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TFnPeriodic> IComponentProcess<Config<TMsg, TFnPeriodic>, TMsg>
    for Component<Config<TMsg, TFnPeriodic>, TMsg>
where
    TMsg: 'static + MsgDataBound,
    TFnPeriodic: 'static + FnMut() -> Vec<TMsg> + Send + Sync,
{
    async fn process(
        &self,
        config: Config<TMsg, TFnPeriodic>,
        msg_bus: MsgBusLinker<TMsg>,
    ) -> Result<(), ComponentError> {
        let msg_bus = msg_bus.init(COMPONENT_NAME);
        fn_process(config, msg_bus).await?;
        Ok(())
    }
}

/// Компонент cmp_inject_periodic
pub type Cmp<TMsg, TFnPeriodic> = Component<Config<TMsg, TFnPeriodic>, TMsg>;
