//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Скомпилировать и загрузить на целевую систему:
//!
//! ```bash
//! cargo build --example cmp_raspberry_i2c_master_pm_di16 --target="aarch64-unknown-linux-gnu" --features="cmp_raspberrypi" --release; scp target/aarch64-unknown-linux-gnu/release/examples/cmp_raspberry_i2c_master_pm_di16 user@target:/home/user/
//! ```
//!
//! Запустить на целевой системе:
//!
//! ```bash
//! sudo ./cmp_raspberry_i2c_master_pm_di16
//! ```

#[cfg(feature = "cmp_raspberrypi")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::{cmp_logger, cmp_raspberrypi_i2c_master},
        drivers_i2c,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{Message, MsgDataBound, MsgKey},
    };
    use serde::{Deserialize, Serialize};
    use tracing::Level;

    tracing_subscriber::fmt().init();

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    pub enum Custom {
        InputStateA0(bool),
        InputStateA1(bool),
        InputStateA2(bool),
        InputStateA3(bool),
        InputStateA4(bool),
        InputStateA5(bool),
        InputStateA6(bool),
        InputStateA7(bool),
        InputStateB0(bool),
        InputStateB1(bool),
        InputStateB2(bool),
        InputStateB3(bool),
        InputStateB4(bool),
        InputStateB5(bool),
        InputStateB6(bool),
        InputStateB7(bool),
    }

    impl MsgDataBound for Custom {}

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    // cmp_raspberrypi_i2c_master ------------------------------------------------------------------
    let config_raspberrypi_i2c_master = cmp_raspberrypi_i2c_master::Config {
        devices: vec![drivers_i2c::I2cDevices::PM_DI16(
            drivers_i2c::pm_di16::Config {
                address: drivers_i2c::I2cSlaveAddress::Direct { slave_address: 10 },
                fn_output_a_0: |value| Some(Message::new_custom(Custom::InputStateA0(value))),
                fn_output_a_1: |value| Some(Message::new_custom(Custom::InputStateA1(value))),
                fn_output_a_2: |value| Some(Message::new_custom(Custom::InputStateA2(value))),
                fn_output_a_3: |value| Some(Message::new_custom(Custom::InputStateA3(value))),
                fn_output_a_4: |value| Some(Message::new_custom(Custom::InputStateA4(value))),
                fn_output_a_5: |value| Some(Message::new_custom(Custom::InputStateA5(value))),
                fn_output_a_6: |value| Some(Message::new_custom(Custom::InputStateA6(value))),
                fn_output_a_7: |value| Some(Message::new_custom(Custom::InputStateA7(value))),
                fn_output_b_0: |value| Some(Message::new_custom(Custom::InputStateB0(value))),
                fn_output_b_1: |value| Some(Message::new_custom(Custom::InputStateB1(value))),
                fn_output_b_2: |value| Some(Message::new_custom(Custom::InputStateB2(value))),
                fn_output_b_3: |value| Some(Message::new_custom(Custom::InputStateB3(value))),
                fn_output_b_4: |value| Some(Message::new_custom(Custom::InputStateB4(value))),
                fn_output_b_5: |value| Some(Message::new_custom(Custom::InputStateB5(value))),
                fn_output_b_6: |value| Some(Message::new_custom(Custom::InputStateB6(value))),
                fn_output_b_7: |value| Some(Message::new_custom(Custom::InputStateB7(value))),
                fn_output_period: Duration::from_millis(100),
            },
        )],
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
