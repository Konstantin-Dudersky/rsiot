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
    async fn process(&self, config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> CmpResult {
        let in_out = in_out.clone_with_new_id("CMP_TEMPLATE", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_i2c_master
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
