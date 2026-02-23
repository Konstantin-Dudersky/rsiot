use async_trait::async_trait;
use esp_idf_svc::hal::{can::CAN, peripheral::Peripheral};

use crate::{
    components_config::can_general::BufferBound,
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_esp_can";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TBuffer, TCan> IComponentProcess<Config<TMsg, TBuffer, TCan>, TMsg>
    for Component<Config<TMsg, TBuffer, TCan>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TCan: Peripheral<P = CAN> + 'static,
    TBuffer: 'static + BufferBound,
{
    async fn process(
        &self,
        config: Config<TMsg, TBuffer, TCan>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> CmpResult {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_can
pub type Cmp<TMsg, TBuffer, TCan> = Component<Config<TMsg, TBuffer, TCan>, TMsg>;
