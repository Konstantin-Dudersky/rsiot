use std::time::Duration;

use rsiot::components::cmp_inject_periodic::*;

use crate::messages::*;

pub fn cmp_counter1() -> Cmp<Msg, impl FnMut() -> Vec<Msg>> {
    let mut counter = 0;
    let config = Config {
        period: Duration::from_millis(2000),
        fn_periodic: move || {
            let msg = Msg::Counter1(counter);
            counter += 1;
            vec![msg]
        },
    };

    Cmp::new(config)
}

pub fn cmp_counter2() -> Cmp<Msg, impl FnMut() -> Vec<Msg>> {
    let mut counter = 0;
    let config = Config {
        period: Duration::from_millis(1000),
        fn_periodic: move || {
            let msg = Msg::Counter2(counter);
            counter += 1;
            vec![msg]
        },
    };

    Cmp::new(config)
}
