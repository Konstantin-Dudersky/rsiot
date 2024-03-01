use async_trait::async_trait;

use rsiot_component_core::{Cache, CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::MsgDataBound;

use crate::config::ConfigAlias;
// #[cfg(feature = "single-thread")]
use crate::fn_process::fn_process;

#[allow(unreachable_code)]
#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMsg> IComponentProcess<ConfigAlias<TMsg>, TMsg> for Component<ConfigAlias<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        _config: ConfigAlias<TMsg>,
        _input: CmpInOut<TMsg>,
        _cache: Cache<TMsg>,
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
        fn_process(in_out.clone_with_new_id("cmp_http_client_wasm"), config)
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

pub type Cmp<TMsg> = Component<ConfigAlias<TMsg>, TMsg>;
