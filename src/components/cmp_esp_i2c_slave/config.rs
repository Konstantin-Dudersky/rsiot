use esp_idf_hal::{gpio::AnyIOPin, i2c::I2c, peripheral::Peripheral};

use crate::message::{Message, MsgDataBound};

/// Функция преобразования входных сообщений в данные для передачи I2C
pub type FnInput<TMsg> = fn(&Message<TMsg>) -> Option<Vec<u8>>;

/// Функция для преобразования полученных данных I2C в исходящие сообщения
pub type FnOutput<TMsg> = fn(Vec<u8>) -> Vec<Message<TMsg>>;

/// Конфигурация cmp_esp_i2c_slave
pub struct Config<TMsg, TI2c, TPeripheral>
where
    TMsg: MsgDataBound,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
{
    /// Ссылка на аппартный интерфейс I2C
    pub i2c: TI2c,

    /// Пин сигнала SDA
    pub sda: AnyIOPin,

    /// Пин сигнала SCL
    pub scl: AnyIOPin,

    /// Адрес на шине I2C
    pub slave_address: u8,

    /// Размер буфера данных, принимаемых от мастера
    pub rx_buf_len: usize,

    /// Размер буфера данных, предостваляемых для мастера
    pub tx_buf_len: usize,

    /// Функция преобразования входных сообщений в данные для передачи I2C
    ///
    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: FnInput<TMsg>,

    /// Функция для преобразования полученных данных I2C в исходящие сообщения
    ///
    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: FnOutput<TMsg>,
}
