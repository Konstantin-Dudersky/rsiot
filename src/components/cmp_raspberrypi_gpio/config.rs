use crate::message::{Message, MsgDataBound};

#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Обработка входов
    pub inputs: Vec<ConfigInput<TMsg>>,

    /// Обработка выходов
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

    use crate::components::cmp_raspberrypi_gpio;

    #[test]
    fn test() {
        let config_raspberrypi_gpio = cmp_raspberrypi_gpio::Config {
            inputs: vec![cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 4,
                fn_output: |value| Message::new_custom(Custom::Input4State(value)),
            }],
            outputs: vec![cmp_raspberrypi_gpio::ConfigOutput {
                pin_number: 2,
                fn_input: |msg| match msg.data {
                    MsgData::Custom(Custom::SetOutput2(value)) => Some(value),
                    _ => None,
                },
            }],
        };
    }
}
