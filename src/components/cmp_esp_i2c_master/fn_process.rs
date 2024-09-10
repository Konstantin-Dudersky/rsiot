use std::sync::Arc;

use esp_idf_hal::{
    i2c::{I2c, I2cDriver},
    peripheral::Peripheral,
};
use esp_idf_svc::hal::{i2c, units::FromValueType};
use tokio::{sync::Mutex, task::JoinSet};

use crate::{drivers_i2c, executor::CmpInOut, message::MsgDataBound};

use super::{rsiot_i2c_driver::RsiotI2cDriver, Config, ConfigBaudrate};

pub async fn fn_process<TMsg, TI2c, TPeripheral>(
    config: Config<TMsg, TI2c, TPeripheral>,
    in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
{
    // Настраиваем I2C
    let baudrate = match config.baudrate {
        ConfigBaudrate::Standard => 100_u32.kHz(),
        ConfigBaudrate::Fast => todo!(),
    };
    let i2c_config = i2c::config::Config::new()
        .baudrate(baudrate.into())
        .sda_enable_pullup(config.pullup_enable)
        .scl_enable_pullup(config.pullup_enable);
    let i2c = I2cDriver::new(config.i2c, config.sda, config.scl, &i2c_config).unwrap();
    let driver = Arc::new(Mutex::new(RsiotI2cDriver::new(i2c)));

    let mut task_set: JoinSet<()> = JoinSet::new();

    for device in config.devices {
        match device {
            // drivers_i2c::I2cDevices::General(config) => {
            //     let device = drivers_i2c::general::Device {
            //         msg_bus: in_out.clone(),
            //         config,
            //         driver: driver.clone(),
            //     };
            //     task_set.spawn(device.spawn());
            // }
            drivers_i2c::I2cDevices::ADS1115 { address, inputs } => {
                let driver = driver.clone();
                let device = drivers_i2c::ads1115::ADS1115 {
                    address,
                    driver,
                    inputs,
                    cmp_in_out: in_out.clone(),
                };
                task_set.spawn(async move { device.spawn().await });
            }

            drivers_i2c::I2cDevices::BMP180 {
                address,
                fn_output,
                oversampling,
            } => {
                let device = drivers_i2c::BMP180 {
                    address,
                    fn_output,
                    oversampling,
                    cmp_in_out: in_out.clone(),
                };
                let driver = driver.clone();
                task_set.spawn(async move { device.fn_process(driver).await });
            }

            drivers_i2c::I2cDevices::DS3231 {
                address,
                fn_input,
                fn_output,
                fn_output_period,
            } => {
                let device = drivers_i2c::ds3231::DS3231 {
                    address,
                    fn_input,
                    fn_output,
                    fn_output_period,
                    in_out: in_out.clone(),
                };
                let driver = driver.clone();
                task_set.spawn(async move { device.spawn(driver).await });
            }

            drivers_i2c::I2cDevices::PCA9555 { address } => {
                let device = drivers_i2c::pca9555::PCA9555 { address };
                let driver = driver.clone();
                task_set.spawn(async move { device.spawn(driver).await });
            }

            drivers_i2c::I2cDevices::PCF8575 {
                address,
                pin_00,
                pin_01,
                pin_02,
                pin_03,
                pin_04,
                pin_05,
                pin_06,
                pin_07,
                pin_10,
                pin_11,
                pin_12,
                pin_13,
                pin_14,
                pin_15,
                pin_16,
                pin_17,
            } => {
                let device = drivers_i2c::PCF8575 {
                    address,
                    pins: vec![
                        pin_00, pin_01, pin_02, pin_03, pin_04, pin_05, pin_06, pin_07, pin_10,
                        pin_11, pin_12, pin_13, pin_14, pin_15, pin_16, pin_17,
                    ],
                };
                let driver = driver.clone();
                let in_out = in_out.clone();
                task_set.spawn(async move { device.fn_process(in_out, driver).await });
            }

            drivers_i2c::I2cDevices::PM_RQ8(config) => {
                let device = drivers_i2c::pm_rq8::Device {
                    msg_bus: in_out.clone(),
                    config,
                    driver: driver.clone(),
                };
                task_set.spawn(device.spawn());
            }

            drivers_i2c::I2cDevices::SSD1306 {} => {
                let device = drivers_i2c::ssd1306::SSD1306 {};
                let driver = driver.clone();
                task_set.spawn(async move { device.fn_process(driver).await });
            }
        }
    }

    while let Some(res) = task_set.join_next().await {
        res.unwrap()
    }

    Ok(())
}
