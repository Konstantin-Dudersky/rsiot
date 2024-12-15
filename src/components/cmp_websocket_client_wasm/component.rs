use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMessage, TService> IComponentProcess<Config<TMessage>, TMessage, TService>
    for Component<Config<TMessage>, TMessage, TService>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMessage>,
        input: CmpInOut<TMessage, TService>,
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
pub type Cmp<TMessage, TService> = Component<Config<TMessage>, TMessage, TService>;
