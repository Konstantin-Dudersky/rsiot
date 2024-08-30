use std::fmt::Debug;

use esp_idf_hal::{gpio::AnyIOPin, i2c::I2c, peripheral::Peripheral};
use serde::{de::DeserializeOwned, Serialize};

use crate::message::{Message, MsgDataBound};

use super::BufferData;

/// Функция преобразования входных сообщений в данные для передачи I2C
pub type FnInput<TMsg, TBufferData> = fn(&Message<TMsg>, &mut TBufferData) -> Option<Vec<u8>>;

/// Функция для преобразования полученных данных I2C в исходящие сообщения
pub type FnOutput<TMsg> = fn(Vec<u8>) -> Vec<Message<TMsg>>;

/// Функция для работы коммуникации I2C
pub type FnI2cComm<TI2cRequest, TI2cResponse> = fn(TI2cRequest) -> anyhow::Result<TI2cResponse>;

/// Конфигурация cmp_esp_i2c_slave
pub struct Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>
where
    TMsg: MsgDataBound,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
    TI2cRequest: Debug + DeserializeOwned + 'static,
    TI2cResponse: Debug + Serialize + 'static,
    TBufferData: BufferData,
{
    /// Ссылка на аппартный интерфейс I2C
    pub i2c: TI2c,

    /// Пин сигнала SDA
    pub sda: AnyIOPin,

    /// Пин сигнала SCL
    pub scl: AnyIOPin,

    /// Адрес на шине I2C
    pub slave_address: u8,

    /// Функция преобразования входных сообщений в данные для передачи I2C
    ///
    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: FnInput<TMsg, TBufferData>,

    /// Функция для преобразования полученных данных I2C в исходящие сообщения
    ///
    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: FnOutput<TMsg>,

    /// Функция для обработки коммуникации с мастером I2C
    pub fn_i2c_comm: FnI2cComm<TI2cRequest, TI2cResponse>,

    /// Структура для хранения буферных данных
    pub buffer_data: TBufferData,
}
