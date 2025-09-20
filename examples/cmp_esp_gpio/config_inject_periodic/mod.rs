use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component};

use crate::messages::*;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut gpio_output_state = false;

    let config = Config {
        period: Duration::from_millis(1000),
        fn_periodic: move || {
            let msg = Msg::GpioOutput(gpio_output_state);
            gpio_output_state = !gpio_output_state;
            vec![msg]
        },
    };

    Cmp::new(config)
}
