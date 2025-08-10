use super::super::{fb_template as fb_main, Config};

use crate::message::example_message::*;

#[test]
fn fn_input() {
    // Заглушка
    let fn_input_0 = |_input: &mut fb_main::I, _msg: &Custom| ();

    let _ = Config::<Custom, fb_main::I, fb_main::Q, fb_main::S> {
        fn_input: fn_input_0,
        ..Default::default()
    };
}
