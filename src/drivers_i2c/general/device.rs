use std::sync::Arc;
use std::time::Duration;

use tokio::{
    sync::{Mutex, MutexGuard},
    time::sleep,
};
use tracing::warn;

use crate::{
    drivers_i2c::{I2cSlaveAddress, RsiotI2cDriverBase},
    executor::CmpInOut,
    message::MsgDataBound,
};

use super::{Config, ConfigRequestKind, Error, FnResponse};

/// Устройство I2C
pub struct Device<TMsg, TDriver>
where
    TMsg: MsgDataBound,
    TDriver: RsiotI2cDriverBase,
{
    /// Внутренняя шина сообщений
    pub msg_bus: CmpInOut<TMsg>,

    /// Конфигурация
    pub config: Config,

    /// Драйвер I2C
    pub driver: Arc<Mutex<TDriver>>,
}

impl<TMsg, TDriver> Device<TMsg, TDriver>
where
    TMsg: MsgDataBound + 'static,
    TDriver: RsiotI2cDriverBase + 'static,
{
    /// Запуск на выполнение
    pub async fn spawn(self) {
        loop {
            sleep(self.config.period).await;
            let mut driver = self.driver.lock().await;
            for (index, req) in self.config.requests.iter().enumerate() {
                // println!("{}", index);
                let response = process_request(
                    req,
                    &mut driver,
                    self.config.address,
                    self.config.timeout,
                    self.config.fn_response,
                    index,
                )
                .await;
                if let Err(err) = response {
                    let err = format!("Request error: {}", err);
                    warn!("{}", err);
                    break;
                }
                sleep(Duration::from_millis(20)).await;
            }
        }
    }
}

async fn process_request<'a, TDriver>(
    req: &ConfigRequestKind,
    driver: &mut MutexGuard<'a, TDriver>,
    address: I2cSlaveAddress,
    timeout: Duration,
    fn_response: FnResponse,
    index: usize,
) -> super::Result<()>
where
    TDriver: RsiotI2cDriverBase + 'static,
{
    match req {
        ConfigRequestKind::Read { response_size } => {
            let mut response = driver
                .read(address, *response_size, timeout)
                .await
                .map_err(Error::Driver)?;
            (fn_response)(index, &mut response).map_err(Error::FnProcess)?;
        }
        ConfigRequestKind::Write { request } => driver
            .write(address, request, timeout)
            .await
            .map_err(Error::Driver)?,
        ConfigRequestKind::WriteRead {
            request,
            response_size,
        } => {
            let mut response = driver
                .write_read(address, request, *response_size, timeout)
                .await
                .map_err(Error::Driver)?;
            (fn_response)(index, &mut response).map_err(Error::FnProcess)?;
        }
    }
    Ok(())
}
