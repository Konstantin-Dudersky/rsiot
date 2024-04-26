//! Драйверы для протокола I2C

mod bmp180;
// mod pcf8575;

mod rsiot_i2c_driver_base;

pub(crate) use bmp180::BMP180;
// pub(crate) use pcf8575::PCF8575;
pub(crate) use rsiot_i2c_driver_base::RsiotI2cDriverBase;

#[derive(Clone)]
pub enum I2cDevices {
    BMP180 {
        /// Адрес. По-умолчанию 0x77
        address: u8,
    },
}
