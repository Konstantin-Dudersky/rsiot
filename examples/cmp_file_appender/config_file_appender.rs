use rsiot::{components::cmp_file_appender::*, executor::Component};

use crate::messages::Msg;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        filename: "examples/cmp_file_appender/test.csv".to_string(),
        fn_input: |msg| match msg {
            Msg::AddLine(v) => ConfigAction::AppendLine(v.clone()),
            Msg::EndProcessing => ConfigAction::EndProcessing,
        },
    };
    Cmp::new(config)
}
