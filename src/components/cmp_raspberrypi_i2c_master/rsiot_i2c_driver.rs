use async_trait::async_trait;
use rppal::i2c::I2c;

use crate::drivers_i2c::RsiotI2cDriverBase;

pub struct RsiotI2cDriver {
    i2c: I2c,
}

impl RsiotI2cDriver {
    pub fn new() -> Result<Self, String> {
        let i2c = I2c::new().map_err(|e| e.to_string())?;
        let i2c = Self { i2c };
        Ok(i2c)
    }
}

#[async_trait]
impl RsiotI2cDriverBase for RsiotI2cDriver {
    async fn read_platform(
        &mut self,
        address: u8,
        response_size: usize,
    ) -> Result<Vec<u8>, String> {
        self.i2c
            .set_slave_address(address as u16)
            .map_err(|e| e.to_string())?;
        let mut response = vec![0; response_size];
        self.i2c.read(&mut response).map_err(|e| e.to_string())?;
        Ok(response)
    }

    async fn write_platform(&mut self, address: u8, request: &[u8]) -> Result<(), String> {
        self.i2c
            .set_slave_address(address as u16)
            .map_err(|e| e.to_string())?;
        self.i2c.write(request).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn write_read_platform(
        &mut self,
        address: u8,
        request: &[u8],
        response_size: usize,
    ) -> Result<Vec<u8>, String> {
        self.i2c
            .set_slave_address(address as u16)
            .map_err(|e| e.to_string())?;
        let mut response = vec![0; response_size];
        self.i2c
            .write_read(request, &mut response)
            .map_err(|e| e.to_string())?;
        Ok(response)
    }

    // TODO - удалить после проверки

    // async fn read(&mut self, address: u8, response_size: usize) -> Result<Vec<u8>, String> {
    //     self.i2c
    //         .set_slave_address(address as u16)
    //         .map_err(|e| e.to_string())?;
    //     let mut response = vec![0; response_size];
    //     let res = self.i2c.read(&mut response);
    //     match res {
    //         Ok(_) => {
    //             trace!("I2C success response");
    //             Ok(response)
    //         }
    //         Err(err) => {
    //             warn!("I2C error response: {err:?}");
    //             Err(err.to_string())
    //         }
    //     }
    // }

    // async fn write(&mut self, address: u8, request: &[u8]) -> Result<(), String> {
    //     self.i2c
    //         .set_slave_address(address as u16)
    //         .map_err(|e| e.to_string())?;
    //     let res = self.i2c.write(request);
    //     match res {
    //         Ok(_) => {
    //             trace!("I2C success response");
    //             Ok(())
    //         }
    //         Err(err) => {
    //             warn!("I2C error response: {err:?}");
    //             Err(err.to_string())
    //         }
    //     }
    // }

    // async fn write_read(
    //     &mut self,
    //     address: u8,
    //     request: &[u8],
    //     response_size: usize,
    // ) -> Result<Vec<u8>, String> {
    //     self.i2c
    //         .set_slave_address(address as u16)
    //         .map_err(|e| e.to_string())?;
    //     let mut response = vec![0; response_size];
    //     let res = self.i2c.write_read(request, &mut response);
    //     match res {
    //         Ok(_) => {
    //             trace!("I2C success response: {response:?}");
    //             Ok(response)
    //         }
    //         Err(err) => {
    //             warn!("I2C error response: {err:?}");
    //             Err(err.to_string())
    //         }
    //     }
    // }
}
