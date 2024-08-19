//! Пример работы с модулем PCA9555 по I2C
//!
//! cargo run --example cmp_esp_i2c_master_pca9555 --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{
        hal::{i2c::I2cDriver, peripherals::Peripherals, units::FromValueType},
        sys::link_patches,
    };
    use tokio::task::LocalSet;
    use tracing::{level_filters::LevelFilter, Level};

    use rsiot::{
        components::{cmp_esp_i2c_master, cmp_inject_periodic, cmp_logger},
        drivers_i2c,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::configure_logging,
        message::*,
    };

    link_patches();
    configure_logging(LevelFilter::INFO).unwrap();

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        Pin00Input(bool),
        Pin01Input(bool),
        Pin02Output(bool),
    }

    impl MsgDataBound for Custom {}

    impl TimeToLive for Custom {}

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut value = false;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(5),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::Pin02Output(value));
            value = !value;
            vec![msg]
        },
    };

    // ESP -----------------------------------------------------------------------------------------
    let peripherals = Peripherals::take().unwrap();

    // I2C
    let config = esp_idf_svc::hal::i2c::config::Config::new().baudrate(100_u32.kHz().into());

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
        &config,
    )
    .unwrap();

    let devices = vec![drivers_i2c::I2cDevices::PCA9555 {
        address: drivers_i2c::I2cSlaveAddress::Direct {
            slave_address: 0x24,
        },
        // pin_00: drivers_i2c::PCF8575PinMode::Input {
        //     fn_output: |value| {
        //         let msg = Message::new_custom(Custom::Pin00Input(value));
        //         Some(msg)
        //     },
        // },
        // pin_01: drivers_i2c::PCF8575PinMode::Input {
        //     fn_output: |value| {
        //         let msg = Message::new_custom(Custom::Pin01Input(value));
        //         Some(msg)
        //     },
        // },
        // pin_02: drivers_i2c::PCF8575PinMode::Output {
        //     fn_input: |msg| match msg.data {
        //         MsgData::Custom(Custom::Pin02Output(value)) => Some(value),
        //         _ => None,
        //     },
        // },
        // pin_03: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_04: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_05: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_06: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_07: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_10: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_11: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_12: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_13: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_14: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_15: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_16: drivers_i2c::PCF8575PinMode::Disabled,
        // pin_17: drivers_i2c::PCF8575PinMode::Disabled,
    }];
    let config_esp_i2c_master = cmp_esp_i2c_master::Config {
        timeout: Duration::from_millis(10000),
        i2c_driver: i2c,
        devices,
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        service: "cmp_esp_example".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            .add_cmp(cmp_esp_i2c_master::Cmp::new(config_esp_i2c_master))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
