use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_linux_gpio
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Конфигурация чтения состояния GPIO
    pub gpio_input: Vec<ConfigGpioInput<TMsg>>,

    /// Конфигурация записи состояния GPIO
    pub gpio_output: Vec<ConfigGpioOutput<TMsg>>,
}

/// Обработка одного выхода
#[derive(Clone)]
pub struct ConfigGpioInput<TMsg> {
    /// Устройство GPIO, например  "/dev/gpiochip0"
    pub dev_gpio: &'static str,

    /// Номер линии GPIO. 0 .. 31
    pub gpio_line: u8,

    /// Описание пина. Выводится командой gpioinfo
    pub description: &'static str,

    /// Преобразование состояния пина в сообщение
    pub fn_gpio_input: fn(bool) -> TMsg,
}

/// Обработка одного выхода
#[derive(Clone)]
pub struct ConfigGpioOutput<TMsg> {
    /// Устройство GPIO, например  "/dev/gpiochip0"
    pub dev_gpio: &'static str,

    /// Номер линии GPIO. 0 .. 31
    pub gpio_line: u8,

    /// Описание пина. Выводится командой gpioinfo
    pub description: &'static str,

    /// Преобразование входящего сообщения в состояние пина
    pub fn_gpio_output: fn(TMsg) -> Option<bool>,

    /// Состояние пина при запуске программы
    pub default_state: bool,
}
