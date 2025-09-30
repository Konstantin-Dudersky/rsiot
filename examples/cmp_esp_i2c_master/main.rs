//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp_wifi --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(feature = "cmp_esp")]
mod config_esp_i2c_master;
#[cfg(feature = "cmp_esp")]
mod config_logger;
#[cfg(feature = "cmp_esp")]
mod messages;

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{hal::peripherals::Peripherals, sys::link_patches};
    use tokio::task::LocalSet;
    use tracing::level_filters::LevelFilter;

    use rsiot::{
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::LogConfig,
    };

    use messages::*;

    link_patches();

    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    // ESP -----------------------------------------------------------------------------------------
    let peripherals = Peripherals::take().unwrap();

    let i2c0 = peripherals.i2c0;
    let pin_sda = peripherals.pins.gpio0.into();
    let pin_scl = peripherals.pins.gpio1.into();

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Msg>::new(executor_config)
            .add_cmp(config_logger::cmp())
            .add_cmp(config_esp_i2c_master::cmp(i2c0, pin_sda, pin_scl))
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
