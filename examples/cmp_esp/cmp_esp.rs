//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(feature = "cmp_esp")]
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
        components::{
            cmp_esp_gpio, cmp_esp_http_server, cmp_esp_mqtt_client, cmp_esp_wifi,
            cmp_inject_periodic, cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::configure_logging,
        message::*,
    };

    link_patches();
    configure_logging(LevelFilter::INFO).unwrap();

    // service -------------------------------------------------------------------------------------
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Service {
        cmp_esp_example,
    }

    impl ServiceBound for Service {}

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    pub enum Custom {
        BootButton(bool),
        Relay0(bool),
        Analog3(f32),
        WiFiConnected,
    }

    impl MsgDataBound for Custom {
        type TService = Service;

        fn define_enabled_routes(&self) -> MsgRoute<Self::TService> {
            MsgRoute::default()
        }

        fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
            TimeToLiveValue::Infinite
        }
    }

    // cmp_http_server_esp -------------------------------------------------------------------------
    let http_server_esp_config = cmp_esp_http_server::Config {
        port: 8010,
        get_endpoints: vec![],
        put_endpoints: vec![],
    };

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize_data()?)),
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut value = false;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(5),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::Relay0(value));
            value = !value;
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
        client: Some(cmp_esp_wifi::ConfigClient {
            ssid: "KonstantinAP".into(),
            password: "Admin123!".into(),
            auth_method: cmp_esp_wifi::ConfigAuthMethod::WPA2Personal,
        }),
    };

    // GPIO
    let gpio_config = cmp_esp_gpio::Config {
        inputs: vec![cmp_esp_gpio::ConfigGpioInput {
            peripherals: peripherals.pins.gpio0.into(),
            fn_output: |value| Message::new_custom(Custom::BootButton(value)),
            pull: cmp_esp_gpio::Pull::Down,
        }],
        outputs: vec![cmp_esp_gpio::ConfigGpioOutput {
            peripherals: peripherals.pins.gpio1.into(),
            fn_input: |msg| match msg.data {
                MsgData::Custom(Custom::Relay0(value)) => Some(value),
                _ => None,
            },
            is_low_triggered: false,
        }],
    };

    // ADC
    // let config_esp_adc = cmp_esp_adc::Config::<Custom> {
    //     adc1: peripherals.adc1,
    //     adc2: peripherals.adc2,
    //     inputs: vec![cmp_esp_adc::ConfigInput {
    //         peripherals: cmp_esp_adc::ConfigInputType::Gpio2(peripherals.pins.gpio2),
    //         attenuation: cmp_esp_adc::ConfigInputAttenuation::Db11,
    //         update_period: Duration::from_secs(1),
    //         fn_output: |value| {
    //             let value = value as f32 / 1000.0;
    //             Message::new_custom(Custom::Analog3(value))
    //         },
    //     }],
    // };

    // MQTT
    let _config_esp_mqtt_client = cmp_esp_mqtt_client::Config::<Custom> {
        client_id: "cmp_esp_example".into(),
        host: "195.43.142.106".into(),
        port: 1883,
        fn_input: |msg| Ok(Some(msg.serialize()?.into_bytes())),
        fn_output: |payload: &[u8]| {
            let payload = String::from_utf8_lossy(payload);
            let msg = Message::deserialize(&payload)?;
            Ok(Some(msg))
        },
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        service: Service::cmp_esp_example,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom, Service>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_esp_http_server::Cmp::new(http_server_esp_config))
            .add_cmp(cmp_esp_wifi::Cmp::new(wifi_config))
            .add_cmp(cmp_esp_gpio::Cmp::new(gpio_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            // .add_cmp(cmp_esp_adc::Cmp::new(config_esp_adc))
            // .add_cmp(cmp_esp_mqtt_client::Cmp::new(config_esp_mqtt_client))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
