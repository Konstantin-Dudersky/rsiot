use std::time::Duration;

use esp_idf_svc::hal::gpio::AnyIOPin;

use crate::message::MsgDataBound;

/// Конфигурация компонента измерения скорости
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Пин, к которому подключен датчик скорости
    pub pin_speed: AnyIOPin<'static>,

    /// Опциональный пин, к которому подключен индикатор состояния
    pub pin_led: Option<AnyIOPin<'static>>,

    /// Время между измерениями скорости
    pub period: Duration,

    /// Частота в герцах
    pub fn_output: fn(f64) -> TMsg,
}
