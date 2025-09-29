use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_http_client";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(&self, config: Config<TMsg>, msgbus_linker: CmpInOut<TMsg>) -> CmpResult {
        fn_process(msgbus_linker.init(COMPONENT_NAME), config).await?;
        Ok(())
    }
}

/// Компонент cmp_http_client
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
