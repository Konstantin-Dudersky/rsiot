//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_plc --target x86_64-unknown-linux-gnu --features="cmp_plc"
//! ```

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod fb1_example;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod fb2_example;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod message;

#[cfg(all(feature = "cmp_plc", not(feature = "single-thread")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use tracing::Level;

    use rsiot::{
        components::{cmp_logger, cmp_plc},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::Message,
    };

    use message::Data;

    tracing_subscriber::fmt().init();

    let plc_config = cmp_plc::Config {
        fn_cycle_init: |_input: &mut fb1_example::I| {},
        fn_input: |_input: &mut fb1_example::I, msg: &Message<Data>| {
            let msg = match msg.get_custom_data() {
                Some(val) => val,
                None => return,
            };
            match msg {
                Data::OutputValue(_) => (),
            }
        },
        fn_output: |data: &fb1_example::Q| {
            let msg = Message::new_custom(Data::OutputValue(data.out_counter));
            vec![msg]
        },
        fb_main: fb1_example::FB::new(),
        period: Duration::from_secs(2),
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "plc-multi-thread".into(),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<message::Data>::new(executor_config)
        .add_cmp(cmp_plc::Cmp::new(plc_config))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(any(not(feature = "cmp_plc"), feature = "single-thread"))]
fn main() {}
