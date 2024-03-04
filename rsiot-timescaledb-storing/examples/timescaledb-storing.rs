//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-timescaledb-storing --example timescaledb-storing
//! ```

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;
    use url::Url;

    use rsiot_component_core::{ComponentExecutor, ComponentExecutorConfig};
    use rsiot_extra_components::cmp_inject_periodic;
    use rsiot_messages_core::{example_message::*, *};
    use rsiot_timescaledb_storing::cmp_timescaledb_storing;

    tracing_subscriber::fmt().init();

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::ValueInstantF64(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    let url = Url::parse("postgres://postgres:postgres@localhost:5432/db_data_test")?;
    let db_config = cmp_timescaledb_storing::Config {
        connection_string: url,
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "timescaledb-storing".into(),
        fn_auth: |_| None,
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_timescaledb_storing::Cmp::new(db_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
