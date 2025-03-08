use serde::{Deserialize, Serialize};

use crate::{
    components::cmp_raspberrypi_gpio,
    message::{example_service::Service, *},
};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
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
    // Пример записи  выхода 2
    let outputs_0 = vec![cmp_raspberrypi_gpio::ConfigOutput {
        pin_number: 2,
        fn_input: |msg| match msg.data {
            MsgData::Custom(Custom::SetOutput2(value)) => Some(value),
            _ => None,
        },
        is_low_triggered: false,
    }];

    for outputs in [outputs_0] {
        let _ = cmp_raspberrypi_gpio::Config {
            outputs,
            ..Default::default()
        };
    }
}
