use async_trait::async_trait;

use crate::{
    executor::{MsgBusLinker, CmpResult, Component, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_mqtt_client";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(&self, config: Config<TMsg>, msg_bus: MsgBusLinker<TMsg>) -> CmpResult {
        let msg_bus = msg_bus.init(COMPONENT_NAME);
        fn_process(config, msg_bus).await?;
        Ok(())
    }
}

/// Компонент cmp_mqtt_client
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
