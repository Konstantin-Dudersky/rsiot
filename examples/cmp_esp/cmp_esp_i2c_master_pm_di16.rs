//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp_i2c_master_pm_di16 --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{hal::peripherals::Peripherals, sys::link_patches};
    use tokio::task::LocalSet;
    use tracing::{level_filters::LevelFilter, Level};

    use rsiot::{
        components::{cmp_esp_i2c_master, cmp_logger},
        drivers_i2c,
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

    impl MsgDataBound for Custom {
        type TService = Service;
    }

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize_data()?)),
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
        devices: vec![drivers_i2c::I2cDevices::PM_DI16(
            drivers_i2c::pm_di16::Config {
                address: drivers_i2c::I2cSlaveAddress::Direct { slave_address: 10 },
                fn_output_a_0: |value| Message::new_custom(Custom::InputStateA0(value)),
                fn_output_a_1: |value| Message::new_custom(Custom::InputStateA1(value)),
                fn_output_a_2: |value| Message::new_custom(Custom::InputStateA2(value)),
                fn_output_a_3: |value| Message::new_custom(Custom::InputStateA3(value)),
                fn_output_a_4: |value| Message::new_custom(Custom::InputStateA4(value)),
                fn_output_a_5: |value| Message::new_custom(Custom::InputStateA5(value)),
                fn_output_a_6: |value| Message::new_custom(Custom::InputStateA6(value)),
                fn_output_a_7: |value| Message::new_custom(Custom::InputStateA7(value)),
                fn_output_b_0: |value| Message::new_custom(Custom::InputStateB0(value)),
                fn_output_b_1: |value| Message::new_custom(Custom::InputStateB1(value)),
                fn_output_b_2: |value| Message::new_custom(Custom::InputStateB2(value)),
                fn_output_b_3: |value| Message::new_custom(Custom::InputStateB3(value)),
                fn_output_b_4: |value| Message::new_custom(Custom::InputStateB4(value)),
                fn_output_b_5: |value| Message::new_custom(Custom::InputStateB5(value)),
                fn_output_b_6: |value| Message::new_custom(Custom::InputStateB6(value)),
                fn_output_b_7: |value| Message::new_custom(Custom::InputStateB7(value)),
                fn_output_period: Duration::from_millis(100),
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
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_esp_i2c_master::Cmp::new(config_esp_i2c_master))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
