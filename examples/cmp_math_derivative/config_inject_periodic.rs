use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component, message::ValueTime};

use super::messages::*;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 1.0;

    let config = Config {
        period: Duration::from_millis(100),
        fn_periodic: move || {
            let msg = Msg::InputValue(ValueTime {
                value: counter,
                time: time::OffsetDateTime::now_utc(),
            });
            counter += 1.0;
            vec![msg]
        },
    };

    Cmp::new(config)
}
