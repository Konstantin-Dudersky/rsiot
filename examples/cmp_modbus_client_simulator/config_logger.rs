use rsiot::{components::cmp_logger::*, executor::Component};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Msg::ValueWrite(v) => format!("Value write: {}", v),
                Msg::ValueRead(v) => format!("Value read: {}", v),
            };

            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
