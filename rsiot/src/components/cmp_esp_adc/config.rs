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
    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub adc1: ADC1,
    pub adc2: ADC2,

    pub inputs: Vec<ConfigItem<TMsg>>,
}

pub struct ConfigItem<TMsg>
where
    TMsg: MsgDataBound,
{
    pub peripherals: ConfigInput,

    pub fn_output: fn(u16) -> Vec<Message<TMsg>>,
}

pub enum ConfigInput {
    Gpio0(gpio::Gpio0),
    Gpio1(gpio::Gpio1),
    Gpio2(gpio::Gpio2),
    Gpio3(gpio::Gpio3),
    Gpio4(gpio::Gpio4),
    Gpio5(gpio::Gpio5),
}
