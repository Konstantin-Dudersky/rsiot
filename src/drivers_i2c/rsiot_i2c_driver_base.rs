use std::marker::Send;

use async_trait::async_trait;
use tracing::{trace, warn};

#[async_trait]
pub trait RsiotI2cDriverBase
where
    Self: Send,
{
    async fn mux_control(&mut self, address: I2cSlaveAddress) -> Result<u8, String> {
        match address {
            I2cSlaveAddress::Direct { slave_address } => Ok(slave_address),
            I2cSlaveAddress::Mux {
                mux_address,
                channel,
                slave_address,
            } => {
                let request = [2_u8.pow(channel as u32)];
                self.write_platform(mux_address, &request).await?;
                Ok(slave_address)
            }
        }
    }

    async fn read(&mut self, address: u8, response_size: usize) -> Result<Vec<u8>, String> {
        let response = self.read_platform(address, response_size).await;
        match response {
            Ok(response) => {
                trace!("I2C success response");
                Ok(response)
            }
            Err(err) => {
                warn!("I2C error response: {err:?}");
                Err(err.to_string())
            }
        }
    }

    async fn write(&mut self, address: I2cSlaveAddress, request: &[u8]) -> Result<(), String> {
        let address = self.mux_control(address).await?;
        let response = self.write_platform(address, request).await;
        match response {
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

    async fn write_read(
        &mut self,
        address: I2cSlaveAddress,
        request: &[u8],
        response_size: usize,
    ) -> Result<Vec<u8>, String> {
        let address = self.mux_control(address).await?;
        let response = self
            .write_read_platform(address, request, response_size)
            .await;
        match response {
            Ok(response) => {
                trace!("I2C success response");
                Ok(response)
            }
            Err(err) => {
                warn!("I2C error response: {err:?}");
                Err(err.to_string())
            }
        }
    }

    async fn read_platform(&mut self, address: u8, response_size: usize)
        -> Result<Vec<u8>, String>;

    async fn write_platform(&mut self, address: u8, request: &[u8]) -> Result<(), String>;

    async fn write_read_platform(
        &mut self,
        address: u8,
        request: &[u8],
        response_size: usize,
    ) -> Result<Vec<u8>, String>;
}

/// Адрес подчиненного устройства
#[derive(Clone, Copy)]
pub enum I2cSlaveAddress {
    /// Прямое подключение
    Direct {
        /// Адрес подчиненного устройства
        slave_address: u8,
    },
    /// Через мультиплексор
    Mux {
        /// Адрес мультиплексора
        mux_address: u8,
        /// Канал на мультиплексоре. 0..7
        channel: u8,
        /// Адрес подчиненного устройства
        slave_address: u8,
    },
}
