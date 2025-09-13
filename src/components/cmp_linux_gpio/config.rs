use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_linux_gpio
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Конфигурация чтения состояния GPIO
    pub read: Vec<ConfigRead<TMsg>>,

    /// Конфигурация записи состояния GPIO
    pub write: Vec<ConfigWrite<TMsg>>,
}

/// Обработка одного выхода
#[derive(Clone)]
pub struct ConfigRead<TMsg> {
    /// Устройство GPIO, например  "/dev/gpiochip0"
    pub dev_gpio: String,

    /// Номер линии GPIO. 0 .. 31
    pub gpio_line: u8,

    /// Преобразование состояния пина в сообщение
    pub fn_read: fn(bool) -> Option<TMsg>,
}

/// Обработка одного выхода
#[derive(Clone)]
pub struct ConfigWrite<TMsg> {
    /// Устройство GPIO, например  "/dev/gpiochip0"
    pub dev_gpio: String,

    /// Номер линии GPIO. 0 .. 31
    pub gpio_line: u8,

    /// Преобразование входящего сообщения в состояние пина
    pub fn_write: fn(TMsg) -> Option<bool>,
}
