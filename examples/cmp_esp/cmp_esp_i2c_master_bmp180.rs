//! Пример работы с модулем PCF8575 по I2C
//!
//! cargo run --example cmp_esp_i2c_master_bmp180 --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{hal::peripherals::Peripherals, sys::link_patches};
    use tokio::task::LocalSet;
    use tracing::{info, level_filters::LevelFilter, Level};

    use rsiot::{
        components::{cmp_esp_i2c_master, cmp_logger},
        drivers_i2c,
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
        cmp_esp_i2c_master_bmp180,
    }

    impl ServiceBound for Service {}

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        VoltageA0(f64),
    }

    impl MsgDataBound for Custom {
        type TService = Service;

        fn define_enabled_routes(&self) -> Vec<(Option<Self::TService>, Option<Self::TService>)> {
            vec![]
        }

        fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
            TimeToLiveValue::Infinite
        }
    }

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    // ESP -----------------------------------------------------------------------------------------
    let peripherals = Peripherals::take().unwrap();

    let devices = vec![drivers_i2c::I2cDevices::BMP180 {
        address: drivers_i2c::I2cSlaveAddress::Direct {
            slave_address: 0x77,
        },
        fn_output: |data| {
            info!("{data:?}");
            vec![]
        },
        oversampling: drivers_i2c::BMP180Oversampling::HighResolution,
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
        service: Service::cmp_esp_i2c_master_bmp180,
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
