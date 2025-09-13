use std::time::Duration;

use rsiot::{components::cmp_math::*, executor::Component, message::ValueTime};
use tracing::info;

use super::messages::*;

pub fn cmp() -> Component<Config<Msg, IntMsg>, Msg> {
    let config = Config {
        fn_input: |msg| match msg {
            Msg::InputValue(v) => Some(IntMsg::InputValue(v)),
            Msg::OutputValue(_) => None,
        },
        fn_output: |int_msg| match int_msg {
            IntMsg::InputValue(_) => None,
            IntMsg::OutputValue(v) => {
                info!("Output value: {v:.2}");
                Some(vec![Msg::OutputValue(v)])
            }
        },
        algs: vec![Algs::Derivative {
            fn_input_value: |int_msg| match int_msg {
                IntMsg::InputValue(v) => Some((v.value, v.time)),
                IntMsg::OutputValue(_) => None,
            },
            fn_input_time_window: |_| Some(Duration::from_millis(1000)),
            normalization_time: Duration::from_secs(3600),
            gamma: Gamma::Derivative,
            fn_output: |output_value| IntMsg::OutputValue(output_value.derivative),
        }],
    };

    Cmp::new(config)
}

#[derive(Clone, Copy, Debug)]
pub enum IntMsg {
    InputValue(ValueTime),
    OutputValue(f64),
}

impl IntMsgBound for IntMsg {}
