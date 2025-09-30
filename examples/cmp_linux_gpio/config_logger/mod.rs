use rsiot::{components::cmp_logger::*, executor::Component};

use crate::message::Msg;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            match msg {
                Msg::InputState(v) => Ok(Some(format!("Input state: {v}"))),
                _ => Ok(None),
            }
        },
    };

    Cmp::new(config)
}
