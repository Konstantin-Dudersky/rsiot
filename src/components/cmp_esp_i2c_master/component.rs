use async_trait::async_trait;
use esp_idf_svc::hal::i2c::I2c;

use crate::{
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_esp_i2c_master";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TI2c> IComponentProcess<Config<TMsg, TI2c>, TMsg> for Component<Config<TMsg, TI2c>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TI2c: I2c + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TI2c>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> CmpResult {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_i2c_master
pub type Cmp<TMsg, TI2c> = Component<Config<TMsg, TI2c>, TMsg>;
