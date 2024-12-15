//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Скомпилировать и загрузить на целевую систему:
//!
//! ```bash
//! cargo build --example cmp_raspberry_i2c_master_pm_rq8 --target="aarch64-unknown-linux-gnu" --features="cmp_raspberrypi" --release; scp target/aarch64-unknown-linux-gnu/release/examples/cmp_raspberry_i2c_master_pm_rq8 user@target:/home/user/
//! ```
//!
//! Запустить на целевой системе:
//!
//! ```bash
//! sudo ./cmp_raspberry_i2c_master_pm_rq8
//! ```

#[cfg(feature = "cmp_raspberrypi")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_logger, cmp_raspberrypi_i2c_master},
        drivers_i2c,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::Service, Message, MsgDataBound},
    };
    use serde::{Deserialize, Serialize};
    use tracing::Level;

    tracing_subscriber::fmt().init();

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        Counter(u8),
    }

    impl MsgDataBound for Custom {
        type TService = Service;
    }

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter: u8 = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::Counter(counter));
            counter += 1;
            vec![msg]
        },
    };

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    // cmp_raspberrypi_i2c_master ------------------------------------------------------------------
    let config_raspberrypi_i2c_master = cmp_raspberrypi_i2c_master::Config {
        devices: vec![drivers_i2c::I2cDevices::PM_RQ8(
            drivers_i2c::pm_rq8::Config {
                address: drivers_i2c::I2cSlaveAddress::Direct { slave_address: 20 },
                fn_input: |msg, buffer| {
                    let Some(msg) = msg.get_custom_data() else {
                        return;
                    };
                    match msg {
                        Custom::Counter(data) => buffer.output_0 = data % 2 == 0,
                    }
                },
            },
        )],
    };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<Custom, Service>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_raspberrypi_i2c_master::Cmp::new(
            config_raspberrypi_i2c_master,
        ))
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_raspberrypi"))]
fn main() {}
