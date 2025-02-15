//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Запустить на машине для разработки:
//!
//! ```bash
//! cargo run --example cmp_linux_serial_master --target="x86_64-unknown-linux-gnu" --features="cmp_linux_uart_master"
//! ```
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

#[cfg(feature = "cmp_linux_uart_master")]
mod test_device;

#[cfg(feature = "cmp_linux_uart_master")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::{
            cmp_linux_uart_master::{self},
            cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::Service, MsgDataBound},
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
    let config_linux_uart = cmp_linux_uart_master::Config::<_, 32> {
        // port: "/dev/ttyAMA0",
        port: "/dev/ttyUSB0",
        baudrate: cmp_linux_uart_master::Baudrate::_9_600,
        data_bits: cmp_linux_uart_master::DataBits::_8,
        stop_bits: cmp_linux_uart_master::StopBits::_1,
        parity: cmp_linux_uart_master::Parity::None,
        timeout: Duration::from_millis(50),
        gpio_chip: "/dev/gpiochip0",
        // pin_rts: Some(17),
        pin_rts: None,
        devices: vec![Box::new(test_device::TestDevice {
            address: 1,
            fn_input: |_, _| (),
            fn_output: |_data| vec![],
        })],
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
        .add_cmp(cmp_linux_uart_master::Cmp::new(config_linux_uart))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_linux_uart_master"))]
fn main() {}
