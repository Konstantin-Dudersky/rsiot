use async_trait::async_trait;

use rsiot_component_core::{
    Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::IMessage;
use tracing::error;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage> IComponentProcess<ConfigAlias<TMessage>, TMessage>
    for Component<ConfigAlias<TMessage>, TMessage>
where
    TMessage: IMessage + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage>,
        input: CmpInput<TMessage>,
        output: CmpOutput<TMessage>,
        _cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        error!("Websocket client component begin execution");
        let config = config.0;
        fn_process(input, output, config).await?;
        error!("Websocket client component end execution");
        Ok(())
    }
}

pub type Cmp<TMessage> = Component<ConfigAlias<TMessage>, TMessage>;
