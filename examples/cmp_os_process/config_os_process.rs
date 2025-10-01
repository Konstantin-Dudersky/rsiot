use rsiot::{components::cmp_os_process::*, executor::Component};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        commands: vec![Command {
            fn_input: |msg| None,
            fn_output: |s| None,
        }],
    };

    Cmp::new(config)
}
