//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Скомпилировать и загрузить на целевую систему:
//!
//! ```bash
//! cargo build --example cmp_linux_serial_master --target="aarch64-unknown-linux-gnu" --features="cmp_linux_serial_master" --release; scp target/aarch64-unknown-linux-gnu/release/examples/cmp_linux_serial_master user@target:/home/user/
//! ```
//!
//! Запустить на целевой системе:
//!
//! ```bash
//! ./cmp_linux_serial_master
//! ```

#[cfg(feature = "cmp_linux_serial_master")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::{
            cmp_linux_serial_master::{self, devices::TestDevice},
            cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::Service, Message, MsgDataBound},
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

    // cmp_linux_uart ------------------------------------------------------------------------------
    let var_name = cmp_linux_serial_master::Config {
        port: "/dev/ttyAMA0",
        baudrate: cmp_linux_serial_master::Baudrate::_115_200,
        data_bits: cmp_linux_serial_master::DataBits::_8,
        stop_bits: cmp_linux_serial_master::StopBits::_1,
        parity: cmp_linux_serial_master::Parity::None,
        delay_between_write_and_read: Duration::from_millis(10),
        fn_input: |_| None,
        fn_output: |_| vec![],
        devices: vec![TestDevice {
            address: 1,
            fn_esp_counter: |data| Message::new_custom(Custom::Pin00Input(false)),
        }],
    };
    let config_linux_uart = var_name;

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        delay_publish: Duration::from_millis(100),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_linux_serial_master::Cmp::new(config_linux_uart))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_linux_serial_master"))]
fn main() {}
