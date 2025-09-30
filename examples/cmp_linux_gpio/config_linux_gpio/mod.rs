use rsiot::{components::cmp_linux_gpio::*, executor::Component};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        gpio_input: vec![ConfigGpioInput {
            dev_gpio: "/dev/gpiochip0",
            gpio_line: 23,
            description: "GPIO Input 1",
            fn_gpio_input: |state| Msg::InputState(state),
        }],
        gpio_output: vec![ConfigGpioOutput {
            dev_gpio: "/dev/gpiochip0",
            gpio_line: 22,
            description: "test_output",
            fn_gpio_output: |msg| match msg {
                Msg::InputState(_) => None,
                Msg::SetOutput(v) => Some(v),
            },
            default_state: false,
        }],
    };

    Cmp::new(config)
}
