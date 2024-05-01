//! Пример работы с модулем PCF8575 по I2C
//!
//! cargo run --example cmp_esp_i2c_master_ads1115 --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

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
        components::{cmp_esp_i2c_master, cmp_logger},
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
        VoltageA0(f64),
    }

    impl MsgDataBound for Custom {}

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
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

    let devices = vec![drivers_i2c::I2cDevices::ADS1115 {
        address: 0x48,
        inputs: vec![drivers_i2c::ads1115::config::InputConfig {
            mux_config: drivers_i2c::ads1115::config::MuxConfig::Single_0,
            amplifier: drivers_i2c::ads1115::config::Amplifier::V_4_096,
            fn_output: |value| Some(Message::new_custom(Custom::VoltageA0(value))),
            period: Duration::from_secs(2),
        }],
    }];

    let config_esp_i2c_master = cmp_esp_i2c_master::Config {
        timeout: Duration::from_millis(10000),
        i2c_driver: i2c,
        devices,
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
            .add_cmp(cmp_esp_i2c_master::Cmp::new(config_esp_i2c_master))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
