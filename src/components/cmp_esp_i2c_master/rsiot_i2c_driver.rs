use esp_idf_svc::hal::i2c::I2cDriver;
use tracing::{trace, warn};

use crate::drivers_i2c::RsiotI2cDriverBase;

/// Адрес подчиненного устройства
pub enum I2cSlaveAddress {
    /// Прямое подключение
    Direct(u8),
    /// Через мультиплексор
    I2cMux { mux_address: u8, slave_address: u8 },
}

pub struct RsiotI2cDriver {
    i2c: I2cDriver<'static>,
}

impl RsiotI2cDriver {
    pub fn new(i2c: I2cDriver<'static>) -> Self {
        Self { i2c }
    }
}

impl RsiotI2cDriverBase for RsiotI2cDriver {
    async fn write_read(
        &mut self,
        address: u8,
        request: &[u8],
        response_size: usize,
    ) -> Result<Vec<u8>, String> {
        let mut response = vec![0; response_size];
        let res = self.i2c.write_read(address, request, &mut response, 1000);
        match res {
            Ok(_) => {
                trace!("I2C success response: {response:?}");
                Ok(response)
            }
            Err(err) => {
                warn!("I2C error response: {err:?}");
                Err(err.to_string())
            }
        }
    }

    async fn write(&mut self, address: u8, request: &[u8]) -> Result<(), String> {
        let res = self.i2c.write(address, request, 1000);
        match res {
            Ok(_) => {
                trace!("I2C success response");
                Ok(())
            }
            Err(err) => {
                warn!("I2C error response: {err:?}");
                Err(err.to_string())
            }
        }
    }

    async fn read(&mut self, address: u8, response_size: usize) -> Result<Vec<u8>, String> {
        let mut response = vec![0; response_size];
        let res = self.i2c.read(address, &mut response, 1000);
        match res {
            Ok(_) => {
                trace!("I2C success response");
                Ok(response)
            }
            Err(err) => {
                warn!("I2C error response: {err:?}");
                Err(err.to_string())
            }
        }
    }
}
