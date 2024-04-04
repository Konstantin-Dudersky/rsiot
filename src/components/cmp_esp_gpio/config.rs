use esp_idf_svc::hal::gpio::{AnyInputPin, AnyOutputPin};

use crate::message::{Message, MsgDataBound};

/// Конфигурация cmp_esp_gpio
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Конфигурация входов
    pub inputs: Vec<ConfigGpioInput<TMsg>>,

    /// Конфигурация выходов
    pub outputs: Vec<ConfigGpioOutput<TMsg>>,
}

/// Конфигурация одного входа
pub struct ConfigGpioInput<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Пин
    pub peripherals: AnyInputPin,

    /// Функция преобразования значения пина в сообщение
    pub fn_output: fn(bool) -> Message<TMsg>,
}

/// Конфигурация одного выхода
pub struct ConfigGpioOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Пин
    pub peripherals: AnyOutputPin,

    /// Функция преобразования входящих сообщений в сигналы управления
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: fn(Message<TMsg>) -> Option<bool>,

    /// Подается ли напряжения в отключенном состоянии или нет
    pub is_low_triggered: bool,
}
