use rsiot::components::cmp_logger::*;

use crate::messages::*;

pub fn cmp() -> Cmp<Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Msg::DeriveMessage { counter1, counter2 } => {
                    format!("counter1: {counter1}, counter2: {counter2}")
                }
                _ => return Ok(None),
            };

            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
