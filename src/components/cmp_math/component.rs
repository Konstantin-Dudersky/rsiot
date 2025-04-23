use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process, IntMsgBound};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TIntMsg> IComponentProcess<Config<TMsg, TIntMsg>, TMsg>
    for Component<Config<TMsg, TIntMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TIntMsg: IntMsgBound + 'static,
{
    async fn process(&self, config: Config<TMsg, TIntMsg>, msg_bus: CmpInOut<TMsg>) -> CmpResult {
        let in_out = msg_bus.clone_with_new_id("cmp_math", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_math
pub type Cmp<TMsg, TIntMsg> = Component<Config<TMsg, TIntMsg>, TMsg>;
