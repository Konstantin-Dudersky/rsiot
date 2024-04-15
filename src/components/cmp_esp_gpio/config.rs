use esp_idf_svc::hal::gpio::{AnyInputPin, AnyOutputPin};

use crate::message::{Message, MsgDataBound};

/// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_auth.html#config
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_esp_gpio.html#inputs
    pub inputs: Vec<ConfigGpioInput<TMsg>>,
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_esp_gpio.html#outputs
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

#[cfg(test)]
mod tests {
    use esp_idf_svc::hal::peripherals::Peripherals;

    use crate::{
        components::cmp_esp_gpio,
        message::{example_message::*, Message, MsgData},
    };

    #[test]
    fn test() {
        let peripherals = Peripherals::take().unwrap();

        let _gpio_config = cmp_esp_gpio::Config {
            // ANCHOR: inputs
            inputs: vec![cmp_esp_gpio::ConfigGpioInput {
                peripherals: peripherals.pins.gpio9.into(),
                fn_output: |value| Message::new_custom(Custom::EspBootButton(value)),
            }],
            // ANCHOR_END: inputs
            // ANCHOR: outputs
            outputs: vec![cmp_esp_gpio::ConfigGpioOutput {
                peripherals: peripherals.pins.gpio1.into(),
                fn_input: |msg| match msg.data {
                    MsgData::Custom(Custom::EspRelay(value)) => Some(value),
                    _ => None,
                },
                is_low_triggered: false,
            }],
            // ANCHOR_END: outputs
        };
    }
}
