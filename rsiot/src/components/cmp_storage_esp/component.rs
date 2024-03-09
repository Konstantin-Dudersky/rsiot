use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::config::Config;
#[cfg(feature = "single-thread")]
use super::fn_process::fn_process;

#[allow(unreachable_code)]
#[cfg(not(feature = "single-thread"))]
#[async_trait]
impl<TMsg, TStorageData> IComponentProcess<Config<TMsg, TStorageData>, TMsg>
    for Component<Config<TMsg, TStorageData>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TStorageData: std::fmt::Debug + Default + DeserializeOwned + PartialEq + Serialize,
{
    async fn process(
        &self,
        _config: Config<TMsg, TStorageData>,
        _input: CmpInput<TMsg>,
        _output: CmpOutput<TMsg>,
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
    TMsg: MsgDataBound + 'static,
    TStorageData: std::fmt::Debug + Default + DeserializeOwned + PartialEq + Serialize,
{
    async fn process(
        &self,
        config: Config<TMsg, TStorageData>,
        input: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(input, config)
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

pub type Cmp<TMsg, TStorageData> = Component<Config<TMsg, TStorageData>, TMsg>;
