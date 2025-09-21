use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 77;

    let config = Config {
        period: Duration::from_millis(1_000),
        fn_periodic: move || {
            let msg = Msg::Counter(counter);
            counter += 1;
            vec![msg]
        },
    };

    Cmp::new(config)
}
