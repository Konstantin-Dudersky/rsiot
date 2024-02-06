use async_trait::async_trait;

use rsiot_component_core::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::config::Config;
// #[cfg(feature = "single-thread")]
use super::fn_process::fn_process;

#[allow(unreachable_code)]
#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        _config: Config<TMsg>,
        _input: ComponentInput<TMsg>,
        _output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        unimplemented!();
    }
}

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg, TStorageData> IComponentProcess<Config<TMsg, TStorageData>, TMsg>
    for Component<Config<TMsg, TStorageData>, TMsg>
where
    TMsg: IMessage + 'static,
    TStorageData: std::fmt::Debug + Default + DeserializeOwned + PartialEq + Serialize,
{
    async fn process(
        &self,
        config: Config<TMsg, TStorageData>,
        input: ComponentInput<TMsg>,
        output: ComponentOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(input, output, config)
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

pub type Cmp<TMsg, TStorageData> = Component<Config<TMsg, TStorageData>, TMsg>;
