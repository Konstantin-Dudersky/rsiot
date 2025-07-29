//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp_wifi --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(all(feature = "cmp_esp", feature = "log_esp"))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{
        eventloop::EspSystemEventLoop, hal::peripherals::Peripherals, sys::link_patches,
        timer::EspTaskTimerService,
    };
    use tokio::task::LocalSet;
    use tracing::{level_filters::LevelFilter, Level};

    use rsiot::{
        components::{cmp_esp_wifi, cmp_logger},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::LogConfig,
        message::*,
    };

    link_patches();
    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    // message --------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    pub enum Custom {
        WiFiConnected,
    }

    impl MsgDataBound for Custom {
        fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
            TimeToLiveValue::Infinite
        }
    }

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize_data()?)),
    };

    // ESP -----------------------------------------------------------------------------------------
    let peripherals = Peripherals::take().unwrap();
    let event_loop = EspSystemEventLoop::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();

    // wifi
    let wifi_config = cmp_esp_wifi::Config {
        peripherals: peripherals.modem,
        event_loop: event_loop.clone(),
        timer_service,
        access_point: Some(cmp_esp_wifi::ConfigAccessPoint {
            ssid: "test_esp".into(),
        }),
        client: Some(cmp_esp_wifi::ConfigClient {
            ssid: "uVazon".into(),
            password: "Admin123!".into(),
            auth_method: cmp_esp_wifi::ConfigAuthMethod::WPA,
        }),
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_esp_wifi::Cmp::new(wifi_config))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await
}

#[cfg(not(all(feature = "cmp_esp", feature = "log_esp")))]
fn main() {
    unimplemented!()
}
