use crate::message::{Message, MsgDataBound};

/// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_raspberrypi_gpio.html#config
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_raspberrypi_gpio.html#inputs
    pub inputs: Vec<ConfigInput<TMsg>>,

    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_raspberrypi_gpio.html#outputs
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

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};

    use crate::{components::cmp_raspberrypi_gpio, message::*};

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        Input4State(bool),
        SetOutput2(bool),
    }

    impl MsgDataBound for Custom {}

    #[test]
    fn test() {
        let _config_raspberrypi_gpio = cmp_raspberrypi_gpio::Config {
            // ANCHOR: inputs
            inputs: vec![cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 4,
                fn_output: |value| Message::new_custom(Custom::Input4State(value)),
            }],
            // ANCHOR_END: inputs
            // ANCHOR: outputs
            outputs: vec![cmp_raspberrypi_gpio::ConfigOutput {
                pin_number: 2,
                fn_input: |msg| match msg.data {
                    MsgData::Custom(Custom::SetOutput2(value)) => Some(value),
                    _ => None,
                },
            }],
            // ANCHOR_END: outputs
        };
    }
}
