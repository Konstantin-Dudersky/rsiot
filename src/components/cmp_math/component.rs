use async_trait::async_trait;

use crate::{
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{IntMsgBound, config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_math";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TIntMsg> IComponentProcess<Config<TMsg, TIntMsg>, TMsg>
    for Component<Config<TMsg, TIntMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TIntMsg: IntMsgBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TIntMsg>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> CmpResult {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_math
pub type Cmp<TMsg, TIntMsg> = Component<Config<TMsg, TIntMsg>, TMsg>;
