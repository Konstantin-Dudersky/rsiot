use std::time::Duration;

use crate::message::{Message, MsgDataBound};

use super::super::I2cSlaveAddress;

/// Функция преобразования входящих сообщений в данные запроса по I2C
pub type FnInput<TMsg> = fn(&Message<TMsg>) -> anyhow::Result<Option<Vec<u8>>>;

/// Функция преобразования данных ответа I2C в исходящие сообщения
pub type FnOutput<TMsg> = fn(Vec<u8>) -> anyhow::Result<Option<Message<TMsg>>>;

/// Конфигурация
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес
    pub address: I2cSlaveAddress,

    /// Тайм-аут запроса
    pub timeout: Duration,

    /// Функция преобразования входящих сообщений в данные запроса по I2C
    pub fn_input: FnInput<TMsg>,

    /// Функция преобразования данных ответа I2C в исходящие сообщения
    pub fn_output: FnOutput<TMsg>,
}
