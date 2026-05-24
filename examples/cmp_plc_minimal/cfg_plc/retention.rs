use std::time::Duration;

use rsiot::{components::cmp_plc::*, message::Message};

use crate::msg::*;

use super::logic::fb_main;

pub fn retention() -> ConfigRetention<Msg, fb_main::I, fb_main::Q, fb_main::S> {
    ConfigRetention {
        save_period: Duration::from_secs(5),
        fn_export: |input, output, stat| {
            let msg_input = Message::new_custom(Msg::Plc(Plc::Input(input.clone())));
            let msg_output = Message::new_custom(Msg::Plc(Plc::Output(output.clone())));
            let msg_static = Message::new_custom(Msg::Plc(Plc::Static(stat.clone())));

            Some(vec![msg_input, msg_output, msg_static])
        },
        fn_import_static: |msg| {
            let msg = msg.get_custom_data();
            let Some(msg) = msg else { return Ok(None) };
            match msg {
                Msg::Filesystem(Filesystem::MemoryStatic(stat)) => Ok(Some(stat)),
                _ => Ok(None),
            }
        },
        restore_timeout: Duration::from_secs(2),
    }
}
