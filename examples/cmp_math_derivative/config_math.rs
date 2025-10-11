use std::time::Duration;

use rsiot::{components::cmp_math::*, executor::Component};
use tracing::info;

use super::messages::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        branches: vec![ConfigBranch {
            fn_input: |msg| match msg {
                Msg::InputValue(v) => Some(*v),
                Msg::OutputValue(_) => None,
            },
            algs: vec![Algs::Derivative {
                time_window: Duration::from_millis(1000),
                normalization_time: Duration::from_secs(3600),
                gamma: Gamma::Derivative,
                fn_output_msgbus: |_| None,
            }],
            fn_output: |ov| {
                info!("{}", ov.value);
                Some(Msg::OutputValue(*ov))
            },
        }],
    };

    Cmp::new(config)
}
