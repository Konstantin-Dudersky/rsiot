use rsiot::components::cmp_plc::plc::types::Resettable;

use crate::msg::*;

use super::logic::fb_main;

pub fn fn_input(input: &mut fb_main::I, msg: &Msg) {
    match msg {
        Msg::Filesystem(_) => (),
        Msg::Plc(_) => (),
        Msg::MsgInjectPeriodic(msg) => match msg {
            MsgInjectPeriodic::Counter(v) => {
                input.counter = *v;
                input.resettabe_bit = Resettable::new(true);
            }
        },
    }
}
