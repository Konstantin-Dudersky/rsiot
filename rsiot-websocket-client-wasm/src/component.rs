use async_trait::async_trait;

use rsiot_component_core::{Cache, CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::MsgDataBound;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMessage> IComponentProcess<ConfigAlias<TMessage>, TMessage>
    for Component<ConfigAlias<TMessage>, TMessage>
where
    TMessage: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMessage>,
        input: CmpInOut<TMessage>,
        _cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config.0,
            input.clone_with_new_id("cmp_websocket_client_wasm"),
        )
        .await
        .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

pub type Cmp<TMessage> = Component<ConfigAlias<TMessage>, TMessage>;
