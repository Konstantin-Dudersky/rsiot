use rsiot::message::Message;

use super::{logic::fb_main, messages::*};

pub fn fn_output(_output: &fb_main::Q) -> Vec<Message<Msg>> {
    vec![].into_iter().map(|m| Message::new_custom(m)).collect()
}
