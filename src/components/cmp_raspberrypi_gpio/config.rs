use crate::message::{Message, MsgDataBound};

/// Конфигурация компонента cmp_raspberrypi_gpio
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Обработка входов
    ///
    /// **Примеры**
    ///
    /// ```rust
    #[doc = include_str!("./test/config_inputs.rs")]
    /// ```
    pub inputs: Vec<ConfigInput<TMsg>>,

    /// Обработка выходов
    ///
    /// **Примеры**
    ///
    /// ```rust
    #[doc = include_str!("./test/config_outputs.rs")]
    /// ```
    pub outputs: Vec<ConfigOutput<TMsg>>,
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

/// Обработка одного входа
#[derive(Clone)]
pub struct ConfigInput<TMsg> {
    /// Номер пина
    pub pin_number: u8,

    /// Преобразование состояния пина в исходящее сообщение
    pub fn_output: fn(bool) -> Message<TMsg>,

    /// Режим подтяжки резисторов
    pub pull_mode: PullMode,
}

/// Режим подтяжки резисторов
#[derive(Clone)]
pub enum PullMode {
    /// Нет подтяжки
    Floating,
    /// Подтяжка к плюсу питания
    Up,
    /// Подтяжка к минусу питания
    Down,
}

/// Обработка одного выхода
#[derive(Clone)]
pub struct ConfigOutput<TMsg> {
    /// Номер пина
    pub pin_number: u8,

    /// Преобразование входящего сообщения в состояние пина
    pub fn_input: fn(Message<TMsg>) -> Option<bool>,

    /// Подается ли напряжения в отключенном состоянии или нет
    pub is_low_triggered: bool,
}
