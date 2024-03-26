use std::time::Duration;

use esp_idf_svc::hal::{
    adc::{ADC1, ADC2},
    gpio,
};

use crate::message::{Message, MsgDataBound};

/// Конфигурация cmp_esp_adc
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Ссылка на ADC
    ///
    /// Пример:
    ///
    /// ```rust
    /// adc1: peripherals.adc1
    /// ```
    pub adc1: ADC1,

    pub adc2: ADC2,

    /// Конфигурация входов
    pub inputs: Vec<ConfigInput<TMsg>>,
}

/// Настройка
pub struct ConfigInput<TMsg> {
    /// Пин подключения
    ///
    /// Пример:
    ///
    /// ```rust
    /// peripherals: cmp_esp_adc::ConfigInputType::Gpio3(peripherals.pins.gpio3)
    /// ```
    pub peripherals: ConfigInputType,

    /// Коэфициент усиления
    ///
    /// Пример:
    ///
    /// ```rust
    /// attenuation: cmp_esp_adc::ConfigInputAttenuation::Db11
    /// ```
    pub attenuation: ConfigInputAttenuation,

    /// Период обновления значения
    ///
    /// Пример:
    ///
    /// ```rust
    /// update_period: Duration::from_secs(1),
    /// ```
    pub update_period: Duration,

    /// Функция преобразования аналогового значения в исходящее сообщение
    ///
    /// Пример:
    ///
    /// ```rust
    /// fn_output: |value| {
    ///     let value = value as f32 / 1000.0;
    ///     Message::new_custom(Custom::Analog3(value))
    /// },
    /// ```
    pub fn_output: fn(u16) -> Message<TMsg>,
}

/// Пин подключения
#[allow(missing_docs)]
pub enum ConfigInputType {
    Gpio0(gpio::Gpio0),
    Gpio1(gpio::Gpio1),
    Gpio2(gpio::Gpio2),
    Gpio3(gpio::Gpio3),
    Gpio4(gpio::Gpio4),
    Gpio5(gpio::Gpio5),
}

/// Коэффициент усиления
pub enum ConfigInputAttenuation {
    DB6,
    Db11,
}
