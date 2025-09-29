use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::config::Config;
#[cfg(feature = "single-thread")]
use super::fn_process::fn_process;

pub const COMPONENT_NAME: &str = "cmp_storage_esp";

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
        config: Config<TMsg, TStorageData>,
        input: CmpInOut<TMsg>,
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
    TStorageData: std::fmt::Debug + Default + DeserializeOwned + PartialEq + Serialize + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TStorageData>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let (input, output) = in_out.msgbus_input_output(COMPONENT_NAME);
        fn_process(input, output, config)
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

/// Компонент cmp_storage_esp
pub type Cmp<TMsg, TStorageData> = Component<Config<TMsg, TStorageData>, TMsg>;
