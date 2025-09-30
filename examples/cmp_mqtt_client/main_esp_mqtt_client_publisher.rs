#[cfg(feature = "cmp_esp")]
mod config_esp_wifi;
#[cfg(feature = "cmp_esp")]
mod config_inject_periodic;
#[cfg(feature = "cmp_esp")]
mod config_mqtt_client;
#[cfg(feature = "cmp_esp")]
mod message;

#[cfg(feature = "cmp_esp")]
#[allow(dead_code)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use esp_idf_svc::{
        eventloop::EspSystemEventLoop, hal::prelude::Peripherals, sys::link_patches,
        timer::EspTaskTimerService,
    };
    use rsiot::{
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::LogConfig,
    };
    use tokio::task::LocalSet;

    use message::*;
    use tracing::level_filters::LevelFilter;

    // ESP -----------------------------------------------------------------------------------------
    link_patches();

    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    let peripherals = Peripherals::take()?;
    let event_loop = EspSystemEventLoop::take()?;
    let timer_service = EspTaskTimerService::new()?;
    let modem = peripherals.modem;

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Msg>::new(executor_config)
            .add_cmp(config_mqtt_client::publisher::cmp())
            .add_cmp(config_esp_wifi::cmp(modem, event_loop, timer_service))
            .add_cmp(config_inject_periodic::cmp())
            .wait_result()
            .await?;

        Ok(()) as anyhow::Result<()>
    });
    local_set.await;

    Ok(())
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {
    unimplemented!()
}
