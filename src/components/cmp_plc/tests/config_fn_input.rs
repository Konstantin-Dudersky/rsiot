use std::time::Duration;

use super::super::{fb_template, Config};

use crate::message::{example_message::*, Message};

fn fn_input() {
    let fn_input_0 = |_input: &mut fb_template::I, _msg: &Message<Custom>| ();

    let _ = Config::<Custom, fb_template::I, fb_template::Q, fb_template::S> {
        fn_cycle_init: todo!(),
        fn_input: fn_input_0,
        fn_output: todo!(),
        fb_main: fb_template::FB::new(),
        period: Duration::from_millis(100),
        retention: None,
    };
}
