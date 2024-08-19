//! Пример работы с модулем PCF8575 по I2C
//!
//! cargo run --example cmp_esp_i2c_master_ds3231 --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

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
        VoltageA0(f64),
        InjectPeriodic(u8),
    }

    impl MsgDataBound for Custom {}

    impl TimeToLive for Custom {}

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(10),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::InjectPeriodic(counter));
            counter += 1;
            vec![msg]
        },
    };

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize_data()?)),
    };

    // ESP -----------------------------------------------------------------------------------------
    let peripherals = Peripherals::take().unwrap();

    // I2C
    let config = esp_idf_svc::hal::i2c::config::Config::new()
        .baudrate(100_u32.kHz().into())
        .sda_enable_pullup(true)
        .scl_enable_pullup(true);

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio6,
        peripherals.pins.gpio7,
        &config,
    )
    .unwrap();

    let devices = vec![drivers_i2c::I2cDevices::DS3231 {
        address: drivers_i2c::I2cSlaveAddress::Direct {
            slave_address: 0x68,
        },
        fn_input: |msg| {
            let data = msg.get_custom_data()?;
            let input_data = match data {
                Custom::InjectPeriodic(sec) => drivers_i2c::ds3231::InputData {
                    year: sec,
                    month: sec,
                    day: sec,
                    hour: sec,
                    minute: sec,
                    second: sec,
                },
                _ => return None,
            };
            Some(input_data)
        },
        fn_output: |data| {
            println!(
                "{:02}-{:02}-{:02}T{:02}:{:02}:{:02}",
                data.year, data.month, data.day, data.hour, data.minute, data.second
            );
            None
        },
        fn_output_period: Duration::from_secs(5),
    }];

    let config_esp_i2c_master = cmp_esp_i2c_master::Config {
        timeout: Duration::from_millis(10000),
        i2c_driver: i2c,
        devices,
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        service: "cmp_esp_i2c_master_bmp180".into(),
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
