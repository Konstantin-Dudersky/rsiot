use std::fmt::Debug;

use async_trait::async_trait;
use esp_idf_hal::{i2c::I2c, peripheral::Peripheral};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse>
    IComponentProcess<Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse>, TMsg>
    for Component<Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
    TI2cRequest: Debug + DeserializeOwned + 'static,
    TI2cResponse: Debug + Serialize + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse>,
        in_out: CmpInOut<TMsg>,
    ) -> CmpResult {
        let in_out = in_out.clone_with_new_id("cmp_esp_i2c_slave", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_i2c_slave
pub type Cmp<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse> =
    Component<Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse>, TMsg>;
