//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Скомпилировать и загрузить на целевую систему:
//!
//! ```bash
//! cargo build --example cmp_raspberrypi --target="aarch64-unknown-linux-gnu" --features="cmp_raspberrypi" --release; scp target/aarch64-unknown-linux-gnu/release/examples/cmp_raspberrypi user@target:/home/user/
//! ```
//!
//! Запустить на целевой системе:
//!
//! ```bash
//! sudo ./cmp_raspberry
//! ```

mod message;

#[cfg(feature = "cmp_raspberrypi")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_logger, cmp_raspberrypi_gpio},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::*,
    };

    use message::*;
    use tracing::Level;

    tracing_subscriber::fmt().init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let config_raspberrypi_gpio = cmp_raspberrypi_gpio::Config {
        inputs: vec![cmp_raspberrypi_gpio::ConfigInput {
            pin_number: 4,
            fn_output: |value| Message::new_custom(Custom::Input4State(value)),
        }],
        outputs: vec![
            cmp_raspberrypi_gpio::ConfigOutput {
                pin_number: 2,
                fn_input: |msg| match msg.data {
                    MsgData::Custom(Custom::SetOutput2(value)) => Some(value),
                    _ => None,
                },
            },
            cmp_raspberrypi_gpio::ConfigOutput {
                pin_number: 3,
                fn_input: |msg| match msg.data {
                    MsgData::Custom(Custom::SetOutput2(value)) => Some(!value),
                    _ => None,
                },
            },
        ],
    };

    let mut flag = false;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(5),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::SetOutput2(flag));
            flag = !flag;
            vec![msg]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "cmp_raspberrypi".into(),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_raspberrypi_gpio::Cmp::new(config_raspberrypi_gpio))
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_raspberrypi"))]
fn main() {}
