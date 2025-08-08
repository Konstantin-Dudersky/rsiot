use std::time::Duration;

use rsiot::{components::cmp_plc::*, executor::Component};

use super::{
    fn_cycle_init::fn_cycle_init, fn_input::fn_input, fn_output::fn_output, logic::fb_main,
    messages::*, retention::retention,
};

pub fn cmp() -> Component<Config<Msg, fb_main::I, fb_main::Q, fb_main::S>, Msg> {
    let config = Config {
        fn_cycle_init,
        fn_input,
        fn_output,
        fb_main: fb_main::FB::new(),
        period: Duration::from_millis(100),
        retention: Some(retention()),
    };

    Cmp::new(config)
}
