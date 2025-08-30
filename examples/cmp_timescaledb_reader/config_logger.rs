use rsiot::{components::cmp_logger::*, message::Message};

use crate::message::*;

pub fn cmp() -> rsiot::executor::Component<Config<Msg>, Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg: Message<Msg>| {
            let text = msg.serialize_data()?;
            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
