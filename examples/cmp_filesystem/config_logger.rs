use rsiot::{components::cmp_logger, executor::Component};
use tracing::Level;

use super::messages::*;

pub fn new() -> Component<cmp_logger::Config<Msg>, Msg> {
    let config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                Msg::InjPeriodic(InjPeriodic::Increase) => "Increase".into(),
                Msg::Filesystem(Filesystem::Counter(v)) => format!("Filesystem: {v}"),
            };
            Ok(Some(text))
        },
    };
    cmp_logger::Cmp::new(config)
}
