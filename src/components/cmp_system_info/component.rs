use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::{Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_system_info";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        msgbus_linker: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME))
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

/// Компонент cmp_system_info
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
