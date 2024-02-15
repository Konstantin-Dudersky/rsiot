//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-plc --example plc-multi-thread
//! ```

mod fb1_example;
mod fb2_example;
mod message;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use tracing::Level;

    use rsiot_component_core::ComponentExecutor;
    use rsiot_extra_components::cmp_logger;
    use rsiot_messages_core::MsgContent;
    use rsiot_plc::cmp_plc;

    use message::Message;

    tracing_subscriber::fmt().init();

    let plc_config = cmp_plc::Config {
        fn_input: |_input: &mut fb1_example::I, msg: &Message| match msg {
            Message::OutputValue(_) => (),
        },
        fn_output: |data: &fb1_example::Q| {
            let msg = Message::OutputValue(MsgContent::new(data.out_counter));
            vec![msg]
        },
        fb_main: fb1_example::FB::new(),
        period: Duration::from_secs(2),
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };

    ComponentExecutor::<message::Message>::new(100, "plc-multi-thread")
        .add_cmp(cmp_plc::Cmp::new(plc_config))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
