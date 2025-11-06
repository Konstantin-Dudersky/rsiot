use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component};

use crate::messages::Msg;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 0;

    let config = Config {
        period: Duration::from_millis(10),
        fn_periodic: move || {
            let msg = match counter {
                0..100 => Msg::AddLineFile1(counter.to_string()),
                100..200 => Msg::AddLineFile2(counter.to_string()),
                _ => Msg::EndProcessing,
            };
            counter += 1;
            vec![msg]
        },
    };

    Cmp::new(config)
}
