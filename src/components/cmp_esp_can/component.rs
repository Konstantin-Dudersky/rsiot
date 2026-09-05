use async_trait::async_trait;

use crate::{
    components_config::can_general::CanFrame,
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_esp_can";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TFnInput> IComponentProcess<Config<TMsg, TFnInput>, TMsg>
    for Component<Config<TMsg, TFnInput>, TMsg>
where
    TMsg: 'static + MsgDataBound,
    TFnInput: 'static + Fn(&TMsg) -> anyhow::Result<Option<Vec<CanFrame>>> + Send + Sync,
{
    async fn process(
        &self,
        config: Config<TMsg, TFnInput>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> CmpResult {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_can
pub type Cmp<TMsg, TFnInput> = Component<Config<TMsg, TFnInput>, TMsg>;
