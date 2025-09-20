use rsiot::{components::cmp_logger::*, executor::Component};

use crate::messages::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Msg::GpioInput(v) => format!("GPIO input: {}", v),
                _ => return Ok(None),
            };

            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
