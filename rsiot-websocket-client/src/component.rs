use async_trait::async_trait;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::message_v2::MsgDataBound;
use tracing::error;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage> IComponentProcess<ConfigAlias<TMessage>, TMessage>
    for Component<ConfigAlias<TMessage>, TMessage>
where
    TMessage: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage>,
        mut input: CmpInput<TMessage>,
        mut output: CmpOutput<TMessage>,
        _cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_id(&mut input, &mut output, "cmp_websocket_client");
        error!("Websocket client component begin execution");
        let config = config.0;
        fn_process(input, output, config).await?;
        error!("Websocket client component end execution");
        Ok(())
    }
}

pub type Cmp<TMessage> = Component<ConfigAlias<TMessage>, TMessage>;
