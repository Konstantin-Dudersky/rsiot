use std::time::Duration;

use esp_idf_svc::{log::EspLogger, sys::link_patches};
use rsiot::{
    cmp_plc,
    component::{ComponentChain, cmp_inject_periodic, cmp_logger},
    message::msg_types,
};
use tokio::main;

use message::Message;
use tracing::Level;

mod fb_main;
mod message;

#[main(flavor = "current_thread")]
async fn main() {
    link_patches();
    EspLogger::initialize_default();

    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: || {
            let msg = Message::InjectU16(msg_types::Value::new(0));
            vec![msg]
        },
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let text = format!("{msg:?}");
            Ok(Some(text))
        },
    };

    let plc_config = cmp_plc::Config {
        fn_input: |input: &mut fb_main::I, msg: &Message| match msg {
            Message::InjectU16(val) => input.inject_u16 = val.value,

            // no need
            Message::InjectU16Output(_) => (),
        },
        fn_output: |output: &fb_main::Q| {
            vec![Message::InjectU16Output(msg_types::Value::new(
                output.inject_u16,
            ))]
        },
        fb_main: fb_main::FB::new(),
        period: Duration::from_secs(1),
        buffer_size: 10,
    };

    let mut chain = ComponentChain::<Message>::new(
        100,
        vec![
            cmp_inject_periodic::new(inject_config),
            cmp_plc::new(plc_config),
            cmp_logger::new(logger_config),
        ],
    );

    chain.spawn().await;
}
