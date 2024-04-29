//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{
        eventloop::EspSystemEventLoop,
        hal::{i2c::I2cDriver, peripherals::Peripherals, units::FromValueType},
        sys::link_patches,
    };
    use tokio::{task::LocalSet, time::sleep};
    use tracing::{error, info, level_filters::LevelFilter, Level};

    use rsiot::{
        components::{
            cmp_esp_adc, cmp_esp_gpio, cmp_esp_i2c_master, cmp_esp_mqtt_client, cmp_esp_wifi,
            cmp_http_server_esp, cmp_inject_periodic, cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::configure_logging,
        message::*,
    };

    link_patches();
    configure_logging(LevelFilter::INFO).unwrap();

    // message
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        BootButton(bool),
        Relay0(bool),
        Analog3(f32),
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
    let _logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
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
        peripherals: peripherals.modem,
        event_loop: event_loop.clone(),
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
    let config_esp_adc = cmp_esp_adc::Config::<Custom> {
        adc1: peripherals.adc1,
        adc2: peripherals.adc2,
        inputs: vec![cmp_esp_adc::ConfigInput {
            peripherals: cmp_esp_adc::ConfigInputType::Gpio2(peripherals.pins.gpio2),
            attenuation: cmp_esp_adc::ConfigInputAttenuation::Db11,
            update_period: Duration::from_secs(1),
            fn_output: |value| {
                let value = value as f32 / 1000.0;
                Message::new_custom(Custom::Analog3(value))
            },
        }],
    };

    // MQTT
    let config_esp_mqtt_client = cmp_esp_mqtt_client::Config::<Custom> {
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

    // I2C
    let config = esp_idf_svc::hal::i2c::config::Config::new().baudrate(100_u32.kHz().into());

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
        &config,
    )
    .unwrap();

    let config_esp_i2c_master = cmp_esp_i2c_master::Config {
        timeout: Duration::from_millis(10000),
        i2c_driver: i2c,
        devices: vec![
            // cmp_esp_i2c_master::I2cDevices::BMP180 {
            //     address: 0x77,
            //     fn_output: |response| {
            //         info!("Temperature and pressure: {response:?}");
            //         vec![]
            //     },
            //     oversampling: cmp_esp_i2c_master::BMP180Oversampling::HighResolution,
            // },
            cmp_esp_i2c_master::I2cDevices::PCF8575 {
                address: 0x20,
                pin_00: cmp_esp_i2c_master::PCF8575PinMode::Input {
                    fn_output: |value| {
                        info!("Input 00: {value}");
                        None
                    },
                },
                pin_01: cmp_esp_i2c_master::PCF8575PinMode::Input {
                    fn_output: |value| {
                        info!("Input 01: {value}");
                        None
                    },
                },
                pin_02: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_03: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_04: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_05: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_06: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_07: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_10: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_11: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_12: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_13: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_14: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_15: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_16: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
                pin_17: cmp_esp_i2c_master::PCF8575PinMode::Disabled,
            },
        ],
    };

    // let mut flag = false;
    // loop {
    //     println!("i2c call");
    //     let send_bytes = if flag {
    //         vec![0x00, 0x00]
    //         // vec![0xFF, 0xFF]
    //     } else {
    //         vec![0xFF, 0xFF]
    //     };
    //     flag = !flag;
    //     // let mut answer = vec![0x00, 0x00];
    //     let size = 2;
    //     let mut answer = vec![0; size];
    //     // PCF8574
    //     match i2c.write_read(0x20, &send_bytes, &mut answer, 1000) {
    //         // i2c.write(0x20, &send_bytes, 1000).unwrap();
    //         Ok(res) => {
    //             info!("result: {:?}", send_bytes);
    //             info!("answer: {:?}", answer);
    //         }
    //         Err(err) => error!("error: {}", err),
    //     }
    //     sleep(Duration::from_secs(2)).await;
    // }

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        executor_name: "cmp_esp_example".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            // .add_cmp(cmp_logger::Cmp::new(logger_config))
            // .add_cmp(cmp_http_server_esp::Cmp::new(http_server_esp_config))
            // .add_cmp(cmp_esp_wifi::Cmp::new(wifi_config))
            // .add_cmp(cmp_esp_gpio::Cmp::new(gpio_config))
            // .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            // .add_cmp(cmp_esp_adc::Cmp::new(config_esp_adc))
            // .add_cmp(cmp_esp_mqtt_client::Cmp::new(config_esp_mqtt_client))
            .add_cmp(cmp_esp_i2c_master::Cmp::new(config_esp_i2c_master))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
