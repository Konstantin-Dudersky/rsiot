use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};

use crate::message::{Message, MsgDataBound};

use super::super::I2cSlaveAddress;

/// Функция обработки ответа
pub type FnResponse = fn(usize, &mut [u8]) -> Result<(), anyhow::Error>;

pub type FnInput<TMsg> = fn(&Message<TMsg>) -> anyhow::Result<Option<Vec<u8>>>;

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

    pub fn_input: FnInput<TMsg>,

    pub fn_output: FnOutput<TMsg>,
}

/// Конфигурация одного запроса
#[derive(Clone)]
pub enum ConfigRequestKind {
    /// Запрос read
    Read {
        /// Кол-во байт чтения
        response_size: usize,
    },
    /// Запрос write
    Write {
        /// Данные для записи
        request: Vec<u8>,
    },
    /// Запрос write_read
    WriteRead {
        /// Данные для записи
        request: Vec<u8>,
        /// Кол-во байт чтения
        response_size: usize,
    },
}
