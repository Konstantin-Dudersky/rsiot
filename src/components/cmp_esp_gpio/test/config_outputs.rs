use esp_idf_svc::hal::peripherals::Peripherals;

use crate::{
    components::cmp_esp_gpio,
    message::{example_message::*, MsgData},
};

#[test]
#[allow(clippy::single_element_loop)]
fn test() {
    let peripherals = Peripherals::take().unwrap();

    // Пример конфигурации массива выходов
    let outputs_0 = vec![cmp_esp_gpio::ConfigGpioOutput {
        peripherals: peripherals.pins.gpio1.into(),
        fn_input: |msg| match msg.data {
            MsgData::Custom(Custom::EspRelay(value)) => Some(value),
            _ => None,
        },
        is_low_triggered: false,
    }];

    for outputs in [outputs_0] {
        let _gpio_config = cmp_esp_gpio::Config {
            outputs,
            ..Default::default()
        };
    }
}
