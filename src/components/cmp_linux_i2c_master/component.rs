use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(&self, config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> CmpResult {
        let msg_bus =
            msg_bus.clone_with_new_id("cmp_linux_i2c_master", AuthPermissions::FullAccess);
        fn_process(config, msg_bus).await?;
        Ok(())
    }
}

/// Компонент cmp_linux_i2c_master
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
