use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component, message::example_message::*};

pub fn cmp() -> Component<Config<Custom, impl FnMut() -> Vec<Custom>>, Custom> {
    let mut counter = 0.0;
    let config = Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Custom::ValueInstantF64(counter);
            counter += 1.0;
            vec![msg]
        },
    };

    Cmp::new(config)
}
