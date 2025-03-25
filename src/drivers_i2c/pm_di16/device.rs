use std::sync::Arc;

use futures::TryFutureExt;
use tokio::{
    sync::{mpsc::channel, Mutex},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    drivers_i2c::RsiotI2cDriverBase,
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{tasks, Config};

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
        let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

        let (ch_output_to_filter_send, ch_output_to_filter_recv) = channel(50);
        let (ch_filter_to_msgbus_send, ch_filter_to_msgbus_recv) = channel(50);

        // Периодический опрос входов и генерирование сообщений
        let task = tasks::Output {
            output: ch_output_to_filter_send,
            address: self.config.address,
            fn_output_a_0: self.config.fn_output_a_0,
            fn_output_a_1: self.config.fn_output_a_1,
            fn_output_a_2: self.config.fn_output_a_2,
            fn_output_a_3: self.config.fn_output_a_3,
            fn_output_a_4: self.config.fn_output_a_4,
            fn_output_a_5: self.config.fn_output_a_5,
            fn_output_a_6: self.config.fn_output_a_6,
            fn_output_a_7: self.config.fn_output_a_7,
            fn_output_b_0: self.config.fn_output_b_0,
            fn_output_b_1: self.config.fn_output_b_1,
            fn_output_b_2: self.config.fn_output_b_2,
            fn_output_b_3: self.config.fn_output_b_3,
            fn_output_b_4: self.config.fn_output_b_4,
            fn_output_b_5: self.config.fn_output_b_5,
            fn_output_b_6: self.config.fn_output_b_6,
            fn_output_b_7: self.config.fn_output_b_7,
            fn_output_period: self.config.fn_output_period,
            driver: self.driver,
        };
        join_set_spawn(&mut task_set, task.spawn());

        // Фильтрация одинаковых сообщений
        let task = shared_tasks::filter_identical_data::FilterIdenticalData {
            input: ch_output_to_filter_recv,
            output: ch_filter_to_msgbus_send,
        };
        join_set_spawn(
            &mut task_set,
            task.spawn().map_err(super::Error::TaskFilterIdenticalData),
        );

        // Отправка исходящих сообщений
        let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
            input: ch_filter_to_msgbus_recv,
            msg_bus: self.msg_bus,
        };
        join_set_spawn(
            &mut task_set,
            task.spawn().map_err(super::Error::TaskMpscToMsgBus),
        );

        while let Some(res) = task_set.join_next().await {
            res.unwrap().unwrap();
        }
    }
}
