//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp --target="riscv32imc-esp-espidf" --features="cmp_esp" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{
        eventloop::EspSystemEventLoop, hal::peripherals::Peripherals, log::EspLogger,
        sys::link_patches, wifi::EspWifi,
    };
    use tokio::task::LocalSet;
    use tracing::Level;

    use rsiot::{
        components::{
            cmp_esp_gpio, cmp_esp_wifi, cmp_http_server_esp, cmp_inject_periodic, cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::*,
    };

    link_patches();
    EspLogger::initialize_default();

    // message
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        BootButton(bool),
        Relay0(bool),
    }

    impl MsgDataBound for Custom {}

    // cmp_http_server_esp -------------------------------------------------------------------------
    let http_server_esp_config = cmp_http_server_esp::Config {
        port: 8010,
        fn_input: |msg: &Message<Custom>| {
            let text = msg.serialize()?;
            Ok(Some(text))
        },
        fn_output: |text: &str| {
            let msg = Message::deserialize(text)?;
            Ok(Some(msg))
        },
    };

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
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

    // wifi
    let wifi_config = cmp_esp_wifi::Config {
        event_loop: event_loop.clone(),
        driver: EspWifi::new(peripherals.modem, event_loop.clone(), None).unwrap(),
    };

    // GPIO
    let gpio_config = cmp_esp_gpio::Config {
        inputs: vec![cmp_esp_gpio::ConfigGpioInput {
            driver: peripherals.pins.gpio9.into(),
            fn_output: |value| Message::new_custom(Custom::BootButton(value)),
        }],
        outputs: vec![cmp_esp_gpio::ConfigGpioOutput {
            driver: peripherals.pins.gpio0.into(),
            fn_input: |msg| match msg.data {
                MsgData::Custom(Custom::Relay0(value)) => Some(value),
                _ => None,
            },
            is_low_triggered: false,
        }],
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        executor_name: "cmp_esp_example".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_http_server_esp::Cmp::new(http_server_esp_config))
            .add_cmp(cmp_esp_wifi::Cmp::new(wifi_config))
            .add_cmp(cmp_esp_gpio::Cmp::new(gpio_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
