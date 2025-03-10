use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tracing::warn;

use crate::{
    drivers_i2c::RsiotI2cDriverBase,
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
    serde_utils::postcard_serde,
};

use super::Config;

/// Устройство I2C
pub struct Device<TMsg, TService, TDriver>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
    TDriver: RsiotI2cDriverBase,
{
    /// Внутренняя шина сообщений
    pub msg_bus: CmpInOut<TMsg, TService>,

    /// Конфигурация
    pub config: Config<TMsg>,

    /// Драйвер I2C
    pub driver: Arc<Mutex<TDriver>>,
}

impl<TMsg, TService, TDriver> Device<TMsg, TService, TDriver>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound,
    TDriver: RsiotI2cDriverBase + 'static,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) {
        while let Ok(msg) = self.msg_bus.recv_input().await {
            let req = (self.config.fn_input)(&msg);
            let req = match req {
                Ok(val) => val,
                Err(err) => {
                    warn!("Error: {}", err);
                    continue;
                }
            };
            let Some(req) = req else { continue };
            let response;
            {
                let mut driver = self.driver.lock().await;
                response = driver
                    .write_read(
                        self.config.address,
                        &req,
                        postcard_serde::MESSAGE_LEN,
                        Duration::from_millis(500),
                    )
                    .await
            }
            let response = match response {
                Ok(val) => val,
                Err(err) => {
                    warn!("Error: {}", err);
                    continue;
                }
            };
            let _msg = (self.config.fn_output)(response);
        }
    }
}
