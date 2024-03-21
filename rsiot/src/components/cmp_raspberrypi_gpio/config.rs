use crate::message::{Message, MsgDataBound};

/// Конфигурация cmp_raspberrypi_gpio
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Обработка входов
    pub inputs: Vec<ConfigInput<TMsg>>,

    /// Обработка выходов
    pub outputs: Vec<ConfigOutput<TMsg>>,
}

/// Обработка одного входа
#[derive(Clone)]
pub struct ConfigInput<TMsg> {
    /// Номер пина
    pub pin_number: u8,

    /// Преобразование состояния пина в исходящее сообщение
    pub fn_output: fn(bool) -> Message<TMsg>,
}

/// Обработка одного выхода
#[derive(Clone)]
pub struct ConfigOutput<TMsg> {
    /// Номер пина
    pub pin_number: u8,

    /// Преобразование входящего сообщения в состояние пина
    pub fn_input: fn(Message<TMsg>) -> Option<bool>,
}
