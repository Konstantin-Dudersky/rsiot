//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp_http_server --target="riscv32imc-esp-espidf" --features="cmp_esp, log_esp" --release

#[cfg(feature = "cmp_esp")]
mod shared;

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{
        eventloop::EspSystemEventLoop, hal::peripherals::Peripherals, sys::link_patches,
        timer::EspTaskTimerService,
    };
    use rsiot::{
        components::{cmp_esp_http_server, cmp_esp_wifi, cmp_inject_periodic, cmp_logger},
        components_config::http_server::{GetEndpointConfig, PutEndpointConfig},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::LogConfig,
        message::*,
        serde_utils::SerdeAlgKind,
    };
    use tokio::task::LocalSet;
    use tracing::{Level, level_filters::LevelFilter};

    use shared::{ClientToServer, ServerToClient};

    link_patches();
    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    enum Custom {
        Counter(f64),
        CounterFromClient(u8),
        WiFiConnected(bool),
    }

    impl MsgDataBound for Custom {}

    // cmp_http_server_esp -------------------------------------------------------------------------
    let http_server_esp_config = cmp_esp_http_server::Config {
        port: 8010,
        fn_start: |msg| match msg {
            Custom::WiFiConnected(v) => Some(*v),
            _ => None,
        },
        get_endpoints: vec![
            Box::new(GetEndpointConfig {
                serde_alg: SerdeAlgKind::Json,
                path: "/data/test",
                server_to_client_data: ServerToClient::default(),
                fn_input: |msg, data| {
                    if let Custom::Counter(counter) = msg {
                        data.counter = *counter
                    }
                },
            }),
            Box::new(GetEndpointConfig {
                serde_alg: SerdeAlgKind::Json,
                path: "/data/test2",
                server_to_client_data: ServerToClient::default(),
                fn_input: |msg, data| {
                    if let Custom::Counter(counter) = msg {
                        data.counter = *counter
                    }
                },
            }),
        ],
        put_endpoints: vec![Box::new(PutEndpointConfig {
            serde_alg: SerdeAlgKind::Json,
            path: "/enter",
            fn_output: |data: ClientToServer| match data {
                ClientToServer::NoData => None,
                ClientToServer::SetCounterFromClient(data) => {
                    Some(Message::new_custom(Custom::CounterFromClient(data)))
                }
            },
        })],
    };

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                Custom::CounterFromClient(data) => format!("Counter from client: {}", data),
                _ => return Ok(None),
            };

            Ok(Some(text))
        },
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut value = 0.0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_millis(100),
        fn_periodic: move || {
            let msg = Custom::Counter(value);
            value += 1.0;
            vec![msg]
        },
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
        client: None,
        fn_wifi_connected: |v| Custom::WiFiConnected(v),
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_esp_http_server::Cmp::new(http_server_esp_config))
            .add_cmp(cmp_esp_wifi::Cmp::new(wifi_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {
    unimplemented!()
}
