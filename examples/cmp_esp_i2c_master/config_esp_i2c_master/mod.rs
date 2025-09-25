use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, i2c::I2c, peripheral::Peripheral};
use rsiot::components::cmp_esp_i2c_master::*;
use rsiot_devices::i2c;
use tracing::info;

use crate::messages::*;

pub fn cmp<TI2c, TPeripheral>(
    i2c: TI2c,
    pin_sda: AnyIOPin,
    pin_scl: AnyIOPin,
) -> rsiot::executor::Component<Config<Msg, TI2c, TPeripheral>, Msg>
where
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
{
    // MPU6050
    let device = i2c::mpu6050::Device {
        address: 0x68,
        request_period: Duration::from_millis(100),
        fn_output: |buffer| {
            info!(
                "accel_x: {:6.2?}, accel_y: {:6.2?}, accel_z: {:6.2?}",
                buffer.read_data.accel_x, buffer.read_data.accel_y, buffer.read_data.accel_z
            );
            vec![]
        },
        gyro_full_range: i2c::mpu6050::FsSel::_250DPS,
        accel_full_range: i2c::mpu6050::AfsSel::_2G,
        default_calibration_offset_accel_x: -5776,
        default_calibration_offset_accel_y: -2688,
        default_calibration_offset_accel_z: 2052,
        default_calibration_offset_gyro_x: 58,
        default_calibration_offset_gyro_y: -51,
        default_calibration_offset_gyro_z: 24,
        default_calibration_start: false,
    };

    let config = Config {
        i2c,
        sda: pin_sda,
        scl: pin_scl,
        baudrate: ConfigBaudrate::Standard,
        pullup_enable: true,
        timeout: Duration::from_millis(50),
        devices: vec![Box::new(device)],
    };

    Cmp::new(config)
}
