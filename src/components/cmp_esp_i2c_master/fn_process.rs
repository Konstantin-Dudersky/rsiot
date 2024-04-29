use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};

use crate::{drivers_i2c, executor::CmpInOut, message::MsgDataBound};

use super::{rsiot_i2c_driver::RsiotI2cDriver, Config, I2cDevices};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let driver = Arc::new(Mutex::new(RsiotI2cDriver::new(config.i2c_driver)));

    let mut task_set: JoinSet<()> = JoinSet::new();

    for device in config.devices {
        match device {
            I2cDevices::BMP180 {
                address,
                fn_output,
                oversampling,
            } => {
                let device = drivers_i2c::BMP180 {
                    address,
                    fn_output,
                    oversampling,
                };
                let driver = driver.clone();
                task_set.spawn(async move { device.fn_process(driver).await });
            }
            I2cDevices::PCF8575 { address, pin_00 } => {
                let device = drivers_i2c::PCF8575 {
                    address,
                    pins: vec![pin_00],
                };
                let driver = driver.clone();
                let in_out = in_out.clone();
                task_set.spawn(async move { device.fn_process(in_out, driver).await });
            }
        }
    }

    while let Some(res) = task_set.join_next().await {
        res.unwrap()
    }

    Ok(())
}
