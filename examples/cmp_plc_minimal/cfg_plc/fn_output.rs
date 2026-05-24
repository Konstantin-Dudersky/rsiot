use crate::msg::*;

use super::logic::fb_main;

#[allow(clippy::map_identity)]
pub fn fn_output(output: &fb_main::Q) -> Vec<Msg> {
    vec![Msg::Plc(Plc::Output(output.clone()))]
}
