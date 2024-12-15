use async_trait::async_trait;

use crate::message::{AuthPermissions, MsgDataBound, ServiceBound};

use crate::executor::{CmpInOut, CmpResult, Component, ComponentError, IComponentProcess};

use super::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TService> IComponentProcess<Config, TMsg, TService> for Component<Config, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound,
{
    async fn process(&self, config: Config, in_out: CmpInOut<TMsg, TService>) -> CmpResult {
        let in_out = in_out.clone_with_new_id("cmp_auth", AuthPermissions::FullAccess);
        fn_process(config, in_out)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

/// Компонент cmp_auth
pub type Cmp<TMsg, TService> = Component<Config, TMsg, TService>;
