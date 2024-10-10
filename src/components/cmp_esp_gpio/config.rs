use esp_idf_svc::hal::gpio::{AnyIOPin, AnyOutputPin, Pull};

use crate::message::{Message, MsgDataBound};

/// Конфигурация компонента cmp_esp_gpio
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Конфигурация входов
    ///
    /// **Примеры**
    ///
    /// ```rust
    #[doc = include_str!("test/config_inputs.rs")]
    /// ```
    pub inputs: Vec<ConfigGpioInput<TMsg>>,

    /// Конфигурация выходов
    ///
    /// **Примеры**
    ///
    /// ```rust
    #[doc = include_str!("test/config_outputs.rs")]
    /// ```
    pub outputs: Vec<ConfigGpioOutput<TMsg>>,
}

impl<TMsg> Default for Config<TMsg>
where
    TMsg: MsgDataBound,
{
    fn default() -> Self {
        Self {
            inputs: vec![],
            outputs: vec![],
        }
    }
}

/// Конфигурация одного входа
pub struct ConfigGpioInput<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Пин
    pub peripherals: AnyIOPin,

    /// Функция преобразования значения пина в сообщение
    pub fn_output: fn(bool) -> Message<TMsg>,

    /// Подключение резистора подтяжки
    pub pull: Pull,
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
