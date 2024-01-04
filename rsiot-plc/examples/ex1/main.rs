mod fb1_example;
mod message;

use std::time::Duration;

use tokio::main;
use tracing::Level;

use rsiot_component_core::ComponentCollection;
use rsiot_extra_components::cmp_logger;
use rsiot_messages_core::msg_types;
use rsiot_plc::cmp_plc;

use message::Message;

#[main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let plc_config = cmp_plc::Config {
        fn_input: |_input: &mut fb1_example::I, msg: &Message| match msg {
            Message::OutputValue(_) => (),
        },
        fn_output: |data: &fb1_example::Q| {
            let msg = Message::OutputValue(msg_types::Value::new(data.out_counter));
            vec![msg]
        },
        fb_main: fb1_example::FB::new(),
        period: Duration::from_secs(2),
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };
    let mut chain = ComponentCollection::<message::Message>::new(
        100,
        vec![cmp_plc::new(plc_config), cmp_logger::new(logger_config)],
    );

    chain.spawn().await?;
    Ok(())
}
