use std::{sync::Arc, time::Duration};

use futures::TryFutureExt;
use tokio::{
    sync::{Mutex, mpsc::channel},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    drivers_i2c::RsiotI2cDriverBase,
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
};

use super::tasks;

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
    TMsg: MsgDataBound + 'static,
    TDriver: RsiotI2cDriverBase + 'static,
{
    /// Запустить на выполнение
    pub async fn spawn(self) {
        let buffer = super::config::Buffer::default();
        let buffer: Arc<Mutex<super::config::Buffer>> = buffer.into();

        let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

        let (ch_0_send, ch_0_recv) = channel(100);
        let (ch_1_send, ch_1_recv) = channel(100);
        let (ch_2_send, ch_2_recv) = channel(100);

        // Входящие сообщения в канал mpsc
        let task = super::super::msgbus_to_mpsc::MsgBusToMpsc {
            msg_bus: self.msg_bus.clone(),
            output: ch_0_send,
        };
        join_set_spawn(
            &mut task_set,
            "pm_rq8",
            task.spawn().map_err(super::Error::TaskMsgBusToMpsc),
        );

        // Обработка входящих сообщений
        let task = tasks::Input {
            input: ch_0_recv,
            output: ch_1_send.clone(),
            fn_input: self.config.fn_input,
            buffer: buffer.clone(),
        };
        join_set_spawn(&mut task_set, "pm_rq8", task.spawn());

        // Периодическая отправка, для надежности
        let task = tasks::InputPeriodic {
            output: ch_1_send,
            buffer,
            period: Duration::from_millis(1000),
        };
        join_set_spawn(&mut task_set, "pm_rq8", task.spawn());

        // Коммуникация I2C
        let task = tasks::I2cComm {
            input: ch_1_recv,
            output: ch_2_send,
            i2c_driver: self.driver.clone(),
            address: self.config.address,
        };
        join_set_spawn(&mut task_set, "pm_rq8", task.spawn());

        // Обработка ответа
        let task = tasks::Output {
            input: ch_2_recv,
            output: self.msg_bus,
        };
        join_set_spawn(&mut task_set, "pm_rq8", task.spawn());

        while let Some(res) = task_set.join_next().await {
            res.unwrap().unwrap();
        }
    }
}
