use std::time::Duration;

use rsiot::components::cmp_inject_periodic::*;

use crate::message::*;

pub fn cmp() -> rsiot::executor::Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 1.0;

    let config = Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Msg::ValueWrite(counter);
            counter += 1.0;
            vec![msg]
        },
    };

    Cmp::new(config)
}
