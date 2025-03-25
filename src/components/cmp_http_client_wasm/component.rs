//! Компонент HTTP-клиент для платформы wasm

use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[allow(unreachable_code)]
#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        _config: ConfigAlias<TMsg>,
        _input: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        unimplemented!();
        let config = _config.0;
        fn_process(_input, config)
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            in_out.clone_with_new_id("cmp_http_client_wasm", AuthPermissions::FullAccess),
            config,
        )
        .await
        .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

/// Компонент cmp_http_client_wasm
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
