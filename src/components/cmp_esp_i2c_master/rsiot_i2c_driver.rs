use async_trait::async_trait;
use esp_idf_svc::hal::i2c::I2cDriver;

use crate::drivers_i2c::RsiotI2cDriverBase;

pub struct RsiotI2cDriver {
    i2c: I2cDriver<'static>,
}

impl RsiotI2cDriver {
    pub fn new(i2c: I2cDriver<'static>) -> Self {
        Self { i2c }
    }
}

#[async_trait]
impl RsiotI2cDriverBase for RsiotI2cDriver {
    async fn read_platform(
        &mut self,
        address: u8,
        response_size: usize,
    ) -> Result<Vec<u8>, String> {
        let mut response = vec![0; response_size];
        self.i2c
            .read(address, &mut response, 1000)
            .map_err(|e| e.to_string())?;
        Ok(response)
    }

    async fn write_platform(&mut self, address: u8, request: &[u8]) -> Result<(), String> {
        self.i2c
            .write(address, request, 1000)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn write_read_platform(
        &mut self,
        address: u8,
        request: &[u8],
        response_size: usize,
    ) -> Result<Vec<u8>, String> {
        let mut response = vec![0; response_size];
        self.i2c
            .write_read(address, request, &mut response, 1000)
            .map_err(|e| e.to_string())?;
        Ok(response)
    }
}
