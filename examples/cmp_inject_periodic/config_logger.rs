use rsiot::components::cmp_logger::*;

use crate::msg::*;

pub fn cmp() -> Cmp<Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Msg::Counter(v) => format!("{v}"),
            };

            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
