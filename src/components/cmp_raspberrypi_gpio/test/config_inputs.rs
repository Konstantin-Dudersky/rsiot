use serde::{Deserialize, Serialize};

use crate::{
    components::cmp_raspberrypi_gpio,
    message::{example_service::Service, *},
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Input4State(bool),
    SetOutput2(bool),
}

impl MsgDataBound for Custom {
    type TService = Service;
}

#[test]
#[allow(clippy::single_element_loop)]
fn test() {
    // Пример чтения состояния входа 4
    let inputs_0 = vec![cmp_raspberrypi_gpio::ConfigInput {
        pin_number: 4,
        fn_output: |value| Message::new_custom(Custom::Input4State(value)),
        pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
    }];

    for inputs in [inputs_0] {
        let _ = cmp_raspberrypi_gpio::Config {
            inputs,
            ..Default::default()
        };
    }
}
