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

#[cfg(feature = "cmp_plc")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use tracing::Level;

    use rsiot::{
        components::{
            cmp_inject_periodic, cmp_logger,
            cmp_plc::{self, plc::types::Resettable},
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::*, Message},
    };

    use message::Data;

    tracing_subscriber::fmt().init();

    // cmp_inject_periodic -------------------------------------------------------------------------
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(10),
        fn_periodic: move || {
            let msg = Message::new_custom(Data::InjectPeriodic(true));
            vec![msg]
        },
    };

    // cmp_plc -------------------------------------------------------------------------------------

    let plc_config = cmp_plc::Config {
        fn_cycle_init: |_input: &mut fb2_example::I| {},
        fn_input: |input: &mut fb2_example::I, msg: &Message<Data>| {
            let Some(msg) = msg.get_custom_data() else {
                return;
            };
            match msg {
                Data::OutputValue(_) => (),
                Data::InjectPeriodic(_value) => input.resettable = Resettable::new(true),
            }
        },
        fn_output: |_data: &fb2_example::Q| vec![],
        fb_main: fb2_example::FB::new(Duration::from_secs(2)),
        period: Duration::from_secs(2),
        retention: None,
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<message::Data>::new(executor_config)
        .add_cmp(cmp_plc::Cmp::new(plc_config))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_plc"))]
fn main() {}
