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

    /// Функция преобразования входящих сообщений в команды для модуля
    pub fn_output_a_0: FnOutput<TMsg>,
    pub fn_output_a_1: FnOutput<TMsg>,
    pub fn_output_a_2: FnOutput<TMsg>,
    pub fn_output_a_3: FnOutput<TMsg>,
    pub fn_output_a_4: FnOutput<TMsg>,
    pub fn_output_a_5: FnOutput<TMsg>,
    pub fn_output_a_6: FnOutput<TMsg>,
    pub fn_output_a_7: FnOutput<TMsg>,

    pub fn_output_b_0: FnOutput<TMsg>,
    pub fn_output_b_1: FnOutput<TMsg>,
    pub fn_output_b_2: FnOutput<TMsg>,
    pub fn_output_b_3: FnOutput<TMsg>,
    pub fn_output_b_4: FnOutput<TMsg>,
    pub fn_output_b_5: FnOutput<TMsg>,
    pub fn_output_b_6: FnOutput<TMsg>,
    pub fn_output_b_7: FnOutput<TMsg>,

    pub fn_output_period: Duration,
}
