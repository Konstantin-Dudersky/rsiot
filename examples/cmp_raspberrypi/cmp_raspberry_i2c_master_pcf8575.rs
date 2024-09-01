//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Скомпилировать и загрузить на целевую систему:
//!
//! ```bash
//! cargo build --example cmp_raspberry_i2c_master_pcf8575 --target="aarch64-unknown-linux-gnu" --features="cmp_raspberrypi" --release; scp target/aarch64-unknown-linux-gnu/release/examples/cmp_raspberry_i2c_master_pcf8575 user@target:/home/user/
//! ```
//!
//! Запустить на целевой системе:
//!
//! ```bash
//! sudo ./cmp_raspberry_i2c_master_pcf8575
//! ```

#[cfg(feature = "cmp_raspberrypi")]
#[tokio::main]
async fn main() {
    use rsiot::{
        components::{cmp_logger, cmp_raspberrypi_i2c_master},
        drivers_i2c::{self, I2cSlaveAddress},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::Service, Message, MsgData, MsgDataBound},
    };
    use serde::{Deserialize, Serialize};
    use tracing::Level;

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
    let devices = vec![drivers_i2c::I2cDevices::PCF8575 {
        address: I2cSlaveAddress::Direct {
            slave_address: 0x20,
        },
        pin_00: drivers_i2c::PCF8575PinMode::Input {
            fn_output: |value| {
                let msg = Message::new_custom(Custom::Pin00Input(value));
                Some(msg)
            },
        },
        pin_01: drivers_i2c::PCF8575PinMode::Input {
            fn_output: |value| {
                let msg = Message::new_custom(Custom::Pin01Input(value));
                Some(msg)
            },
        },
        pin_02: drivers_i2c::PCF8575PinMode::Output {
            fn_input: |msg| match msg.data {
                MsgData::Custom(Custom::Pin02Output(value)) => Some(value),
                _ => None,
            },
        },
        pin_03: drivers_i2c::PCF8575PinMode::Disabled,
        pin_04: drivers_i2c::PCF8575PinMode::Disabled,
        pin_05: drivers_i2c::PCF8575PinMode::Disabled,
        pin_06: drivers_i2c::PCF8575PinMode::Disabled,
        pin_07: drivers_i2c::PCF8575PinMode::Disabled,
        pin_10: drivers_i2c::PCF8575PinMode::Disabled,
        pin_11: drivers_i2c::PCF8575PinMode::Disabled,
        pin_12: drivers_i2c::PCF8575PinMode::Disabled,
        pin_13: drivers_i2c::PCF8575PinMode::Disabled,
        pin_14: drivers_i2c::PCF8575PinMode::Disabled,
        pin_15: drivers_i2c::PCF8575PinMode::Disabled,
        pin_16: drivers_i2c::PCF8575PinMode::Disabled,
        pin_17: drivers_i2c::PCF8575PinMode::Disabled,
    }];

    let config_raspberrypi_i2c_master = cmp_raspberrypi_i2c_master::Config { devices };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
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
