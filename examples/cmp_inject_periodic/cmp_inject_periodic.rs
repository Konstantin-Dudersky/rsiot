//! Запуск:
//! ```bash
//! cargo run -p rsiot-extra-components --example cmp_inject_periodic
//!
//! cargo run -p rsiot-extra-components --example cmp_inject_periodic --features single-thread
//! ```

#[cfg(feature = "executor")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use tokio::{task::LocalSet, time::Duration};
    use tracing::{level_filters::LevelFilter, Level};

    use rsiot::{
        components::{cmp_inject_periodic, cmp_logger},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, example_service::*, Message},
    };

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Custom::ValueInstantF64(counter);
            let msg = Message::new_custom(msg);
            counter += 1.0;
            vec![msg]
        },
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
            .wait_result()
            .await
            .unwrap();
    });

    local_set.await;

    Ok(())
}

#[cfg(not(feature = "executor"))]
fn main() {}
