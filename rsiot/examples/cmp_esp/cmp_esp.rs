//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp --target="riscv32imc-esp-espidf" --features="cmp_esp" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use esp_idf_svc::{
        eventloop::EspSystemEventLoop,
        hal::{gpio::PinDriver, peripherals::Peripherals},
        log::EspLogger,
        sys::link_patches,
        wifi::EspWifi,
    };
    use tokio::task::LocalSet;
    use tracing::Level;

    use rsiot::{
        components::{cmp_esp_gpio_input, cmp_esp_wifi, cmp_http_server_esp, cmp_logger},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::*,
    };

    link_patches();
    EspLogger::initialize_default();

    // message
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        BootButton(bool),
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

    // ESP -----------------------------------------------------------------------------------------
    let peripherals = Peripherals::take().unwrap();
    let event_loop = EspSystemEventLoop::take().unwrap();

    // wifi
    let wifi_config = cmp_esp_wifi::Config::<Custom> {
        fn_input: |_| None,
        fn_output: |_| vec![],
        event_loop: event_loop.clone(),
        driver: EspWifi::new(peripherals.modem, event_loop.clone(), None).unwrap(),
    };

    // GPIO9 - button Boot
    let gpio9_config = cmp_esp_gpio_input::Config {
        fn_output: |value| Message::new_custom(Custom::BootButton(value)),
        driver: PinDriver::input(peripherals.pins.gpio9).unwrap(),
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        executor_name: "cmp_http_server_esp_example".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_http_server_esp::Cmp::new(http_server_esp_config))
            .add_cmp(cmp_esp_wifi::Cmp::new(wifi_config))
            .add_cmp(cmp_esp_gpio_input::Cmp::new(gpio9_config))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
