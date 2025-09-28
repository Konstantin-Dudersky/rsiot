//! Компонент HTTP-клиент для платформы wasm

use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_http_client_wasm";

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
        msg_bus: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let (input, output) = msg_bus.msgbus_input_output(COMPONENT_NAME);
        fn_process(input, output, config)
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

/// Компонент cmp_http_client_wasm
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
