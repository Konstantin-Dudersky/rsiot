use async_trait::async_trait;

use crate::{
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(&self, config: Config<TMsg>, in_out: MsgBusLinker<TMsg>) -> CmpResult {
        let in_out = in_out.clone_with_new_id("cmp_http_client_esp", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_http_client_esp
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
