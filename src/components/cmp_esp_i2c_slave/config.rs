use std::fmt::Debug;
use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, i2c::I2c, peripheral::Peripheral};
use serde::{de::DeserializeOwned, Serialize};

use crate::message::{Message, MsgDataBound};

use super::BufferData;

/// Функция преобразования входных сообщений в данные для передачи I2C
pub type FnInput<TMsg, TBufferData> = fn(&Message<TMsg>, &mut TBufferData);

/// Функция для преобразования полученных данных I2C в исходящие сообщения
pub type FnOutput<TMsg, TBufferData> = fn(&TBufferData) -> Vec<Message<TMsg>>;

/// Функция для работы коммуникации I2C
pub type FnI2cComm<TI2cRequest, TI2cResponse, TBufferData> =
    fn(TI2cRequest, &mut TBufferData) -> anyhow::Result<TI2cResponse>;

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

    /// Функция для преобразования полученных данных I2C в исходящие сообщения.
    ///
    /// Функция вызывается по времени, с периодом вызова `fn_output_period`
    ///
    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: FnOutput<TMsg, TBufferData>,

    /// Период вызова `fn_output`
    pub fn_output_period: Duration,

    /// Функция для обработки коммуникации с мастером I2C
    pub fn_i2c_comm: FnI2cComm<TI2cRequest, TI2cResponse, TBufferData>,

    /// Структура для хранения буферных данных
    pub buffer_data_default: TBufferData,

    /// Задержка запуска обмена по I2C. Может потребоваться, чтобы подождать инициализации
    /// других устройств, например по шине SPI
    pub start_i2ccomm_delay: Duration,
}
