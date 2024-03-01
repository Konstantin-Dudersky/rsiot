use async_trait::async_trait;

use rsiot_component_core::{Cache, CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::MsgDataBound;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<ConfigAlias<TMsg>, TMsg> for Component<ConfigAlias<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMsg>,
        in_out: CmpInOut<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(in_out.clone_with_new_id("cmp_http_client"), config).await
    }
}

pub type Cmp<TMsg> = Component<ConfigAlias<TMsg>, TMsg>;
