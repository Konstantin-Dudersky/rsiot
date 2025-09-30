use async_trait::async_trait;

use crate::message::{AuthPermissions, MsgDataBound};

use crate::executor::{MsgBusLinkerCmpResult, Component, ComponentError, IComponentProcess, MsgBusLinker};

use super::{Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config, TMsg> for Component<Config, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(&self, config: Config, in_out: MsgBusLinker<TMsg>) -> CmpResult {
        let in_out = in_out.clone_with_new_id("cmp_auth", AuthPermissions::FullAccess);
        fn_process(config, in_out)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

/// Компонент cmp_auth
pub type Cmp<TMsg> = Component<Config, TMsg>;
