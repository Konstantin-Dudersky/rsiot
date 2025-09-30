use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut state = false;

    let config = Config {
        period: Duration::from_millis(1000),
        fn_periodic: move || {
            let msg = Msg::SetOutput(state);
            state = !state;
            vec![msg]
        },
    };

    Cmp::new(config)
}
