use super::{logic::fb_main, messages::*};

pub fn fn_output(_output: &fb_main::Q) -> Vec<Msg> {
    vec![].into_iter().map(|m| m).collect()
}
