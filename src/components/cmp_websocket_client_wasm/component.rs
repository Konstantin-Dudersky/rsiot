use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMessage> IComponentProcess<Config<TMessage>, TMessage>
    for Component<Config<TMessage>, TMessage>
where
    TMessage: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMessage>,
        input: CmpInOut<TMessage>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            input.clone_with_new_id("cmp_websocket_client_wasm", AuthPermissions::FullAccess),
        )
        .await
        .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

/// Компонент cmp_websocket_client_wasm
pub type Cmp<TMessage> = Component<Config<TMessage>, TMessage>;
