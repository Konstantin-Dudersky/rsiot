//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Скомпилировать и загрузить на целевую систему:
//!
//! ```bash
//! cargo build --example cmp_raspberry_i2c_master_ads1115 --target="aarch64-unknown-linux-gnu" --features="cmp_raspberrypi" --release; scp target/aarch64-unknown-linux-gnu/release/examples/cmp_raspberry_i2c_master_ads1115 user@target:/home/user/
//! ```
//!
//! Запустить на целевой системе:
//!
//! ```bash
//! sudo ./cmp_raspberry_i2c_master_ads1115
//! ```

#[cfg(feature = "cmp_raspberrypi")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::{cmp_logger, cmp_raspberrypi_i2c_master},
        drivers_i2c::{self, I2cSlaveAddress},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{Message, MsgDataBound, MsgKey},
    };
    use serde::{Deserialize, Serialize};
    use tracing::Level;

    tracing_subscriber::fmt().init();

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    pub enum Custom {
        VoltageA0(f64),
    }

    impl MsgDataBound for Custom {}

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
                    let text = format!("{msg:?}");
                    Ok(Some(text))
                },
    };

    // cmp_raspberrypi_i2c_master ------------------------------------------------------------------
    let config_raspberrypi_i2c_master = cmp_raspberrypi_i2c_master::Config {
        devices: vec![drivers_i2c::I2cDevices::ADS1115 {
            address: I2cSlaveAddress::Direct {
                slave_address: 0x48,
            },
            inputs: vec![drivers_i2c::ads1115::config::InputConfig {
                mux_config: drivers_i2c::ads1115::config::MuxConfig::Single_0,
                amplifier: drivers_i2c::ads1115::config::Amplifier::V_4_096,
                fn_output: |value| Some(Message::new_custom(Custom::VoltageA0(value))),
                period: Duration::from_secs(2),
            }],
        }],
    };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<Custom>::new(executor_config)
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
