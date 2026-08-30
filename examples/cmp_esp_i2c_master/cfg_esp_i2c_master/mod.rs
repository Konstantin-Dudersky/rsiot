use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, i2c::I2c};
use rsiot::{components::cmp_esp_i2c_master::*, components_config::i2c_master::I2cAddress};
use rsiot_devices::i2c;
use tracing::info;

use crate::msg::*;

pub fn cmp<TI2c>(
    i2c: TI2c,
    pin_sda: AnyIOPin<'static>,
    pin_scl: AnyIOPin<'static>,
) -> Cmp<Msg, TI2c>
where
    TI2c: I2c + 'static,
{
    // MPU6050
    let device = i2c::MPU6050::Device {
        address: I2cAddress::Direct { address: 0x68 },
        request_period: Duration::from_millis(100),
        fn_output: |buffer| {
            info!(
                "accel_x: {:6.2?}, accel_y: {:6.2?}, accel_z: {:6.2?}",
                buffer.read_data.accel_x, buffer.read_data.accel_y, buffer.read_data.accel_z
            );
            vec![]
        },
        gyro_full_range: i2c::MPU6050::GyroFullScale::Deg250,
        accel_full_range: i2c::MPU6050::AccelFullScale::G2,
        calibration_accel_x: -5776,
        calibration_accel_y: -2688,
        calibration_accel_z: 2052,
        calibration_gyro_x: 58,
        calibration_gyro_y: -51,
        calibration_gyro_z: 24,
        start_calibration: false,
        dmp_enabled: false,
    };

    let config = Config {
        i2c,
        sda: pin_sda,
        scl: pin_scl,
        baudrate: ConfigBaudrate::Standard,
        pullup_enable: true,
        timeout: Duration::from_millis(50),
        devices: vec![Box::new(device)],
        fn_diag: |diag| Msg::I2cDiag(diag.clone()),
        fn_diag_period: Duration::from_millis(1000),
    };

    Cmp::new(config)
}
