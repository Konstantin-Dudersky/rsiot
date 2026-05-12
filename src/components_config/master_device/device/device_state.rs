use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::message::MsgDataBound;

/// Состояние устройства
#[derive(Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct DeviceState {
    /// Инициализация завершена
    pub init_completed: bool,

    /// Количество успешных ответов
    pub response_ok_count: usize,

    /// Количество ошибочных ответов
    pub response_err_count: usize,
}

/// Конфигурация вывода состояния устройства
#[derive(Clone, Debug)]
pub struct ConfigDeviceStateOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Функция преобразования состояния устройства в сообщение
    pub fn_device_state: fn(DeviceState) -> TMsg,

    /// Период вывода состояния устройства
    pub period: Duration,
}
