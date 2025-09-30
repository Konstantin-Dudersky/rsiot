use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, task::JoinSet};
use tracing::warn;

use crate::{
    executor::MsgBusLinker,
    message::{Message, MsgDataBound},
};

use super::{
    super::{I2cSlaveAddress, RsiotI2cDriverBase},
    task_input::{InputData, TaskInput},
    task_output::{OutputData, TaskOutput},
};

/// Часы реального времени
pub struct DS3231<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес. По-умолчанию 0x68
    pub address: I2cSlaveAddress,
    /// Функция преобразования входящих сообщений в данные для записи в модуль
    pub fn_input: fn(Message<TMsg>) -> Option<InputData>,
    /// Функция преобразования данных с модуля в исходящие сообщения
    pub fn_output: fn(OutputData) -> Option<Vec<Message<TMsg>>>,
    /// Период чтения данных с часов
    pub fn_output_period: Duration,
    /// Внутренняя шина сообщений
    pub in_out: MsgBusLinker<TMsg>,
}

impl<TMsg> DS3231<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    /// Запустить опрос датчика
    pub async fn spawn(&self, driver: Arc<Mutex<impl RsiotI2cDriverBase + 'static>>) {
        loop {
            let mut task_set: JoinSet<Result<(), String>> = JoinSet::new();

            let task_input = TaskInput {
                address: self.address,
                driver: driver.clone(),
                fn_input: self.fn_input,
                in_out: self.in_out.clone(),
            };
            task_set.spawn(async move { task_input.spawn().await });

            let task_output = TaskOutput {
                address: self.address,
                period: self.fn_output_period,
                driver: driver.clone(),
                fn_output: self.fn_output,
                in_out: self.in_out.clone(),
            };
            task_set.spawn(async move { task_output.spawn().await });

            while let Some(res) = task_set.join_next().await {
                warn!("{res:?}");
                task_set.shutdown().await;
            }
        }
    }
}
