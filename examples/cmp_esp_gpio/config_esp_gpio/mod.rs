use esp_idf_svc::hal::gpio::{AnyIOPin, AnyOutputPin};
use rsiot::{components::cmp_esp_gpio::*, executor::Component};

use crate::messages::*;

pub fn cmp(pin_input: AnyIOPin, pin_output: AnyOutputPin) -> Component<Config<Msg>, Msg> {
    let config = Config {
        inputs: vec![ConfigGpioInput {
            peripherals: pin_input,
            fn_output: |value| Msg::GpioInput(value),
            pull: Pull::Down,
        }],
        outputs: vec![ConfigGpioOutput {
            peripherals: pin_output,
            fn_input: |msg| match msg {
                Msg::GpioOutput(value) => Some(value),
                _ => None,
            },
            default: false,
        }],
    };

    Cmp::new(config)
}
