use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component};

use crate::messages::*;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 0;
    let config = Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Msg::CounterInhectPeriodic(counter);
            counter += 1;
            vec![msg]
        },
    };

    Cmp::new(config)
}
