use std::time::Duration;

use crate::message::{Message, MsgDataBound};

use super::super::I2cSlaveAddress;

/// Конфигурация
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес
    pub address: I2cSlaveAddress,

    /// Функция преобразования полученных данных в сообщения
    pub fn_output: fn(Vec<u8>) -> Vec<Message<TMsg>>,

    /// Период опроса
    pub fn_output_period: Duration,
}
