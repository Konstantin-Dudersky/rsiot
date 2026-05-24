mod fn_cycle_init;
mod fn_input;
mod fn_output;
mod logic;
mod retention;

pub use logic::fb_main;

use std::time::Duration;

use super::msg::*;

use rsiot::components::cmp_plc::*;

pub fn cmp() -> Cmp<Msg, fb_main::I, fb_main::Q, fb_main::S> {
    let config = Config {
        fn_cycle_init: fn_cycle_init::fn_cycle_init,
        fn_input: fn_input::fn_input,
        fn_output: fn_output::fn_output,
        fb_main: fb_main::FB::new(),
        period: Duration::from_millis(100),
        retention: Some(retention::retention()),
    };

    Cmp::new(config)
}
