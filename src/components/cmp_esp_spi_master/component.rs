use async_trait::async_trait;
use esp_idf_svc::hal::spi::{Spi, SpiAnyPins};

use crate::{
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_esp_spi_master";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TSpi> IComponentProcess<Config<TMsg, TSpi>, TMsg> for Component<Config<TMsg, TSpi>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TSpi: Spi + SpiAnyPins + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TSpi>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> CmpResult {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_spi_master
pub type Cmp<TMsg, TSpi, const MESSAGE_LEN: usize> = Component<Config<TMsg, TSpi>, TMsg>;
