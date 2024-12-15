use async_trait::async_trait;
use esp_idf_svc::hal::{i2c::I2c, peripheral::Peripheral};

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TI2c, TPeripheral, TService>
    IComponentProcess<Config<TMsg, TI2c, TPeripheral>, TMsg, TService>
    for Component<Config<TMsg, TI2c, TPeripheral>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TI2c, TPeripheral>,
        in_out: CmpInOut<TMsg, TService>,
    ) -> CmpResult {
        let in_out = in_out.clone_with_new_id("cmp_esp_i2c_master", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_i2c_master
pub type Cmp<TMsg, TI2c, TPeripheral, TService> =
    Component<Config<TMsg, TI2c, TPeripheral>, TMsg, TService>;
