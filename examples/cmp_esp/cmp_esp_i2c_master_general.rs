//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp_i2c_master_general --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{hal::peripherals::Peripherals, sys::link_patches};
    use tokio::task::LocalSet;
    use tracing::{info, level_filters::LevelFilter, Level};

    use rsiot::{
        components::{cmp_esp_i2c_master, cmp_inject_periodic, cmp_logger},
        drivers_i2c,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::configure_logging,
        message::*,
    };

    link_patches();
    configure_logging(LevelFilter::INFO).unwrap();

    // I2C messages --------------------------------------------------------------------------------
    #[derive(Debug, Deserialize, Serialize)]
    pub enum I2cRequest {
        SetCounterFromMaster(u32),
        GetCounterFromSlave,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum I2cResponse {
        Ok,
        CounterFromSlave(u32),
    }

    let request_1 = I2cRequest::GetCounterFromSlave;
    let request_1 = drivers_i2c::postcard_serde::serialize(&request_1).unwrap();

    let request_2 = I2cRequest::SetCounterFromMaster(777);
    let request_2 = drivers_i2c::postcard_serde::serialize(&request_2).unwrap();

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

    // cmp_logger ----------------------------------------------------------------------------------
    let _logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize_data()?)),
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter: u8 = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(5),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::Counter(counter));
            counter += 1;
            vec![msg]
        },
    };

    // ESP -----------------------------------------------------------------------------------------
    let peripherals = Peripherals::take().unwrap();

    // cmp_esp_i2c_slave ---------------------------------------------------------------------------
    let config_esp_i2c_master = cmp_esp_i2c_master::Config {
        i2c: peripherals.i2c0,
        sda: peripherals.pins.gpio0.into(),
        scl: peripherals.pins.gpio1.into(),
        baudrate: cmp_esp_i2c_master::ConfigBaudrate::Standard,
        pullup_enable: true,
        timeout: Duration::from_millis(2000),
        devices: vec![drivers_i2c::I2cDevices::General(
            drivers_i2c::general::Config {
                address: drivers_i2c::I2cSlaveAddress::Direct {
                    slave_address: 0x77,
                },
                timeout: Duration::from_secs(2),
                requests: vec![
                    drivers_i2c::general::ConfigRequestKind::WriteRead {
                        request: request_1,
                        response_size: drivers_i2c::postcard_serde::MESSAGE_LEN,
                    },
                    drivers_i2c::general::ConfigRequestKind::WriteRead {
                        request: request_2,
                        response_size: drivers_i2c::postcard_serde::MESSAGE_LEN,
                    },
                ],
                period: Duration::from_millis(500),
                fn_response: |_index, data| {
                    let response: I2cResponse = drivers_i2c::postcard_serde::deserialize(data)?;
                    info!("Response: {:?}", response);
                    Ok(())
                },
            },
        )],
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
            .add_cmp(cmp_esp_i2c_master::Cmp::new(config_esp_i2c_master))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}