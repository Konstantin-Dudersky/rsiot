//! cargo run  --example cmp_storage_esp --target="riscv32imc-esp-espidf" --features="cmp_storage_esp, single-thread" --release

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{log::EspLogger, sys::link_patches};
    use tokio::task::LocalSet;
    use tracing::Level;

    use rsiot::{
        components::{cmp_esp_nvs, cmp_inject_periodic, cmp_logger},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, example_service::*, *},
    };

    link_patches();
    EspLogger::initialize_default();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter = 0.0;
    let inject_periodic_config = cmp_inject_periodic::Config {
        period: std::time::Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::ValueInstantF64(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    // cmp_storage_esp -----------------------------------------------------------------------------
    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    struct StorageData {
        pub test_f64: f64,
        pub test_i32: i32,
    }

    let storage_config = cmp_esp_nvs::Config {
        fn_input: |data: &StorageData, msg: &Message<Custom>| match msg.data {
            MsgData::Custom(Custom::ValueInstantF64(value)) => Some(StorageData {
                test_f64: value,
                ..*data
            }),
            _ => None,
        },
        fn_output: |data: &StorageData| {
            vec![Message::new(MsgData::Custom(Custom::ValueInstantF64(
                data.test_f64,
            )))]
        },
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_esp_nvs::Cmp::new(storage_config))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}
