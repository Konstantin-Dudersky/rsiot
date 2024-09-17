use std::time::Duration;

use crate::{
    drivers_i2c::I2cSlaveAddress,
    message::{Message, MsgDataBound},
};

pub type FnOutput<TMsg> = fn(bool) -> Message<TMsg>;

/// Настройка модулуля коммуникации с модулем PM-DI16
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес модуля
    pub address: I2cSlaveAddress,

    /// Функция преобразования значений входов в исходящие сообщения для входа a.0
    pub fn_output_a_0: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа a.1
    pub fn_output_a_1: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа a.2
    pub fn_output_a_2: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа a.3
    pub fn_output_a_3: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа a.4
    pub fn_output_a_4: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа a.5
    pub fn_output_a_5: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа a.6
    pub fn_output_a_6: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа a.7
    pub fn_output_a_7: FnOutput<TMsg>,

    /// Функция преобразования значений входов в исходящие сообщения для входа b.0
    pub fn_output_b_0: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа b.1
    pub fn_output_b_1: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа b.2
    pub fn_output_b_2: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа b.3
    pub fn_output_b_3: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа b.4
    pub fn_output_b_4: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа b.5
    pub fn_output_b_5: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа b.6
    pub fn_output_b_6: FnOutput<TMsg>,
    /// Функция преобразования значений входов в исходящие сообщения для входа b.7
    pub fn_output_b_7: FnOutput<TMsg>,

    /// Период чтения входов
    pub fn_output_period: Duration,
}
