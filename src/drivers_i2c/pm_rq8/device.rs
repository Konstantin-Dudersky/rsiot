use std::{sync::Arc, time::Duration};

use pm_firmware_lib::pm_rq8_v0_0_3::I2cRequest;
use tokio::sync::Mutex;
use tracing::warn;

use crate::{
    drivers_i2c::{postcard_serde, RsiotI2cDriverBase},
    executor::CmpInOut,
    message::MsgDataBound,
};

use super::Config;

/// Модуль PM-RQ8
pub struct Device<TMsg, TDriver>
where
    TMsg: MsgDataBound,
    TDriver: RsiotI2cDriverBase,
{
    /// Внутренняя шина сообщений
    pub msg_bus: CmpInOut<TMsg>,

    /// Конфигурация
    pub config: Config<TMsg>,

    /// Драйвер I2C
    pub driver: Arc<Mutex<TDriver>>,
}

impl<TMsg, TDriver> Device<TMsg, TDriver>
where
    TMsg: MsgDataBound,
    TDriver: RsiotI2cDriverBase,
{
    /// Запустить на выполнение
    pub async fn spawn(mut self) {
        let mut buffer = super::config::Buffer::default();

        while let Ok(msg) = self.msg_bus.recv_input().await {
            (self.config.fn_input)(&msg, &mut buffer);
            let buffer_u8 = buffer.clone().into();
            let request = I2cRequest::SetOutputs(buffer_u8);
            let request = postcard_serde::serialize(&request).unwrap();
            let response;
            {
                let mut driver = self.driver.lock().await;
                response = driver
                    .write_read(
                        self.config.address,
                        &request,
                        postcard_serde::MESSAGE_LEN,
                        Duration::from_millis(500),
                    )
                    .await
            }
            let _response = match response {
                Ok(val) => val,
                Err(err) => {
                    warn!("Error: {}", err);
                    continue;
                }
            };
        }
    }
}
