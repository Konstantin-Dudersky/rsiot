use std::{sync::Arc, time::Duration};

use tokio::sync::{Mutex, MutexGuard};
use tracing::warn;

use crate::drivers_i2c::{postcard_serde, I2cSlaveAddress, RsiotI2cDriverBase};

use super::{I2cRequest, I2cResponse, TaskInput, TaskOutput};

pub struct I2cComm<TDriver> {
    pub input: TaskInput<I2cRequest>,
    pub output: TaskOutput<I2cResponse>,
    pub i2c_driver: Arc<Mutex<TDriver>>,
    pub address: I2cSlaveAddress,
}

impl<TDriver> I2cComm<TDriver>
where
    TDriver: RsiotI2cDriverBase,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(request) = self.input.recv().await {
            let response = {
                let mut i2c_driver = self.i2c_driver.lock().await;
                process_request(request, &mut i2c_driver, self.address).await
            };
            let response = match response {
                Ok(val) => val,
                Err(err) => {
                    warn!("I2C comm error: {}", err);
                    continue;
                }
            };
            self.output
                .send(response)
                .await
                .map_err(|_| super::Error::TokioTaskSend)?;
        }
        Err(super::Error::TaskI2cComm)
    }
}

async fn process_request<'a, TDriver>(
    request: I2cRequest,
    i2c_driver: &mut MutexGuard<'a, TDriver>,
    address: I2cSlaveAddress,
) -> super::Result<I2cResponse>
where
    TDriver: RsiotI2cDriverBase,
{
    let request = postcard_serde::serialize(&request)?;
    let mut response = {
        i2c_driver
            .write_read(
                address,
                &request,
                postcard_serde::MESSAGE_LEN,
                Duration::from_millis(100),
            )
            .await
            .map_err(|e| super::Error::I2c(e.to_string()))?
    };
    let response: I2cResponse = postcard_serde::deserialize(&mut response)?;
    Ok(response)
}
