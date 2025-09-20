//! Пример работы с модулем DS3231 по I2C
//!
//! cargo run --example cmp_esp_i2c_master_ds3231 --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(all(feature = "cmp_esp", feature = "log_esp"))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{hal::peripherals::Peripherals, sys::link_patches};
    use tokio::task::LocalSet;
    use tracing::{Level, level_filters::LevelFilter};

    use rsiot::{
        components::{cmp_esp_i2c_master, cmp_inject_periodic, cmp_logger},
        drivers_i2c,
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

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    pub enum Custom {
        VoltageA0(f64),
        InjectPeriodic(u8),
    }

    impl MsgDataBound for Custom {
        fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
            TimeToLiveValue::Infinite
        }
    }

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(10),
        fn_periodic: move || {
            let msg = Custom::InjectPeriodic(counter);
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
        timeout: Duration::from_millis(1000),
        i2c: peripherals.i2c0,
        sda: peripherals.pins.gpio4.into(),
        scl: peripherals.pins.gpio5.into(),
        baudrate: cmp_esp_i2c_master::ConfigBaudrate::Standard,
        pullup_enable: true,
        devices,
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
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            .add_cmp(cmp_esp_i2c_master::Cmp::new(config_esp_i2c_master))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(all(feature = "cmp_esp", feature = "log_esp")))]
fn main() {
    unimplemented!()
}
