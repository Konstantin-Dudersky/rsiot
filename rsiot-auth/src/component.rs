use async_trait::async_trait;

use rsiot_component_core::{
    Cache, CmpInOut, Component, ComponentError, ComponentResult, IComponentProcess,
};
use rsiot_messages_core::{AuthPermissions, MsgDataBound};

use crate::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config, TMsg> for Component<Config, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config,
        in_out: CmpInOut<TMsg>,
        cache: Cache<TMsg>,
    ) -> ComponentResult {
        let in_out = in_out.clone_with_new_id("cmp_auth", AuthPermissions::FullAccess);
        fn_process(config, cache, in_out)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

pub type Cmp<TMsg> = Component<Config, TMsg>;
