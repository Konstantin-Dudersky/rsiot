use std::time::Duration;

use rsiot::{components::cmp_linux_can::*, executor::Component};

use super::messages::*;

pub fn cmp() -> Component<Config<Msg, ()>, Msg> {
    let config = Config {
        dev_can: "vcan0".into(),
        buffer_default: (),
        fn_input: |_, _| Ok(None),
        period: Duration::from_millis(1000),
        fn_periodic: |_| Ok(None),
        fn_output: |_| None,
    };

    Cmp::new(config)
}
