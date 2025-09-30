use std::fmt::Debug;

use async_trait::async_trait;
use esp_idf_svc::hal::{i2c::I2c, peripheral::Peripheral};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{BufferData, config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_esp_i2c_slave";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>
    IComponentProcess<Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>, TMsg>
    for Component<Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
    TI2cRequest: Debug + DeserializeOwned + 'static,
    TI2cResponse: Debug + Serialize + 'static,
    TBufferData: BufferData + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> CmpResult {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_i2c_slave
pub type Cmp<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData> =
    Component<Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>, TMsg>;
