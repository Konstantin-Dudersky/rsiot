//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp_i2c_slave --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{hal::peripherals::Peripherals, sys::link_patches};
    use tokio::task::LocalSet;
    use tracing::{level_filters::LevelFilter, Level};

    use rsiot::{
        components::{cmp_esp_i2c_slave, cmp_inject_periodic, cmp_logger},
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
        cmp_esp_example,
    }

    impl ServiceBound for Service {}

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        Counter(u8),
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

    // I2C messages --------------------------------------------------------------------------------
    #[derive(Debug, Deserialize, Serialize)]
    pub enum I2cRequest {
        Request1(u32),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum I2cResponse {
        Response1(u32),
    }

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize_data()?)),
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter: u8 = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(1),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::Counter(counter));
            counter += 1;
            vec![msg]
        },
    };

    // ESP -----------------------------------------------------------------------------------------
    let peripherals = Peripherals::take().unwrap();

    // cmp_esp_i2c_slave ---------------------------------------------------------------------------
    let config_esp_i2c_slave = cmp_esp_i2c_slave::Config {
        i2c: peripherals.i2c0,
        sda: peripherals.pins.gpio0.into(),
        scl: peripherals.pins.gpio1.into(),
        slave_address: 0x77,
        buffer_len: 128,
        fn_input: |msg| {
            let msg = msg.get_custom_data()?;
            match msg {
                Custom::Counter(data) => Some(vec![data]),
            }
        },
        fn_output: |_| vec![],
        fn_master_comm: |req: I2cRequest| match req {
            I2cRequest::Request1(data) => Ok(I2cResponse::Response1(data)),
        },
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        service: Service::cmp_esp_example,
        fn_auth: |msg, _| Some(msg),
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            // .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            .add_cmp(cmp_esp_i2c_slave::Cmp::new(config_esp_i2c_slave))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
