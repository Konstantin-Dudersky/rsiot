#[cfg(feature = "cmp_esp")]
mod config_esp_gpio;
#[cfg(feature = "cmp_esp")]
mod config_inject_periodic;
#[cfg(feature = "cmp_esp")]
mod config_logger;
#[cfg(feature = "cmp_esp")]
mod messages;

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // executor ------------------------------------------------------------------------------------

    use std::time::Duration;

    use esp_idf_svc::{hal::prelude::Peripherals, sys::link_patches};
    use rsiot::{executor::ComponentExecutorConfig, logging::LogConfig};
    use tokio::task::LocalSet;

    use messages::*;
    use tracing::level_filters::LevelFilter;

    link_patches();

    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    let peripherals = Peripherals::take().unwrap();

    let pin_input = peripherals.pins.gpio0.into();
    let pin_output = peripherals.pins.gpio1.into();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        use rsiot::executor::ComponentExecutor;

        ComponentExecutor::<Msg>::new(executor_config)
            .add_cmp(config_inject_periodic::cmp())
            .add_cmp(config_esp_gpio::cmp(pin_input, pin_output))
            .add_cmp(config_logger::cmp())
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {
    unimplemented!()
}
