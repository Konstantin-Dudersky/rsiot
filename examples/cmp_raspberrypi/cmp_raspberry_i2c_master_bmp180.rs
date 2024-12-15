//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Скомпилировать и загрузить на целевую систему:
//!
//! ```bash
//! cargo build --example cmp_raspberry_i2c_master_bmp180 --target="aarch64-unknown-linux-gnu" --features="cmp_raspberrypi" --release; scp target/aarch64-unknown-linux-gnu/release/examples/cmp_raspberry_i2c_master_bmp180 user@target:/home/user/
//! ```
//!
//! Запустить на целевой системе:
//!
//! ```bash
//! sudo ./cmp_raspberry_i2c_master_bmp180
//! ```

#[cfg(feature = "cmp_raspberrypi")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::{cmp_logger, cmp_raspberrypi_i2c_master},
        drivers_i2c::{self, I2cSlaveAddress},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::Service, MsgDataBound},
    };
    use serde::{Deserialize, Serialize};
    use tracing::{info, Level};

    tracing_subscriber::fmt().init();

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        Pin00Input(bool),
        Pin01Input(bool),
        Pin02Output(bool),
    }

    impl MsgDataBound for Custom {
        type TService = Service;
    }

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    // cmp_raspberrypi_i2c_master ------------------------------------------------------------------
    let config_raspberrypi_i2c_master = cmp_raspberrypi_i2c_master::Config {
        devices: vec![drivers_i2c::I2cDevices::BMP180 {
            address: I2cSlaveAddress::Direct {
                slave_address: 0x77,
            },
            fn_output: |value| {
                info!("{:?}", value);
                vec![]
            },
            oversampling: drivers_i2c::BMP180Oversampling::HighResolution,
        }],
    };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        delay_publish: Duration::from_millis(100),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom, Service>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_raspberrypi_i2c_master::Cmp::new(
            config_raspberrypi_i2c_master,
        ))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_raspberrypi"))]
fn main() {}
