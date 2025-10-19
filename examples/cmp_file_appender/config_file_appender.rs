use rsiot::{components::cmp_file_appender::*, executor::Component};

use crate::messages::Msg;

pub fn cmp() -> Component<Config<Msg, impl Fn(Msg) -> ConfigAction>, Msg> {
    let config = Config {
        _msg_phantom: std::marker::PhantomData,
        fn_input: |msg| match msg {
            Msg::AddLineFile1(v) => ConfigAction::AppendLine {
                filename: "examples/cmp_file_appender/test_1.csv".to_string(),
                line: v.clone(),
            },
            Msg::AddLineFile2(v) => ConfigAction::AppendLine {
                filename: "examples/cmp_file_appender/test_2.csv".to_string(),
                line: v.clone(),
            },
            Msg::EndProcessing => ConfigAction::EndProcessing,
        },
    };
    Cmp::new(config)
}
