use std::time::Duration;

use rsiot::components::cmp_inject_periodic::*;

use crate::messages::*;

pub fn cmp() -> rsiot::executor::Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 0;

    let config = Config {
        period: Duration::from_millis(10),
        fn_periodic: move || {
            let msg = Msg::Counter(counter);
            counter += 1;
            vec![msg]
        },
    };

    Cmp::new(config)
}
