use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};
use tracing::warn;

use crate::{drivers_i2c::RsiotI2cDriverBase, executor::CmpInOut, message::MsgDataBound};

use super::{tasks, Config};

/// Устройство I2C
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
    TMsg: MsgDataBound + 'static,
    TDriver: RsiotI2cDriverBase + 'static,
{
    /// Запуск на выполнение
    pub async fn spawn(self) {
        let mut task_set: JoinSet<Result<(), String>> = JoinSet::new();

        // let task_input = TaskInput {
        //     address: self.address,
        //     driver: driver.clone(),
        //     fn_input: self.fn_input,
        //     in_out: self.in_out.clone(),
        // };
        // task_set.spawn(async move { task_input.spawn().await });

        let task_output = tasks::Output {
            address: self.config.address,
            driver: self.driver.clone(),
            fn_output: self.config.fn_output,
            msg_bus: self.msg_bus.clone(),
            period: self.config.fn_output_period,
        };
        task_set.spawn(task_output.spawn());

        while let Some(res) = task_set.join_next().await {
            warn!("res: {res:?}");
            task_set.shutdown().await;
        }
    }
}
