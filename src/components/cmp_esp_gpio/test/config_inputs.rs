use esp_idf_svc::hal::peripherals::Peripherals;

use crate::{
    components::cmp_esp_gpio,
    message::{example_message::*, Message},
};

#[test]
#[allow(clippy::single_element_loop)]
fn test() {
    let peripherals = Peripherals::take().unwrap();

    // Пример конфигурации массива входов
    let inputs_0 = vec![cmp_esp_gpio::ConfigGpioInput {
        peripherals: peripherals.pins.gpio9.into(),
        fn_output: |value| Message::new_custom(Custom::EspBootButton(value)),
        pull: cmp_esp_gpio::Pull::Down,
    }];

    for inputs in [inputs_0] {
        let _gpio_config = cmp_esp_gpio::Config {
            inputs,
            ..Default::default()
        };
    }
}
