use async_trait::async_trait;
use esp_idf_svc::hal::{i2c::I2c, peripheral::Peripheral};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TI2c, TPeripheral> IComponentProcess<Config<TMsg, TI2c, TPeripheral>, TMsg>
    for Component<Config<TMsg, TI2c, TPeripheral>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
{
    async fn process(
        &self,
        config: Config<TMsg, TI2c, TPeripheral>,
        in_out: CmpInOut<TMsg>,
    ) -> CmpResult {
        let in_out = in_out.clone_with_new_id("cmp_esp_i2c_master", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_i2c_master
pub type Cmp<TMsg, TI2c, TPeripheral> = Component<Config<TMsg, TI2c, TPeripheral>, TMsg>;
