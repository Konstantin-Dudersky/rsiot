//! Запуск:
//!
//! ```bash
//! RUST_LOG=debug cargo run --example cmp_timescaledb --features "cmp_timescaledb"
//! ```

#[cfg(feature = "cmp_timescaledb")]
mod message;

#[cfg(feature = "cmp_timescaledb")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    use tokio::time::Duration;

    use rsiot::{
        components::{
            cmp_inject_periodic,
            cmp_timescaledb::{self, Row},
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::Message,
    };

    use message::Custom;

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::Counter(counter));
            counter += 1;
            vec![msg]
        },
    };

    let config_timescaledb = cmp_timescaledb::Config {
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data_test".into(),
        max_connections: 5,
        fn_input: |msg| {
            let row = match msg {
                Custom::Counter(v) => Row::new_simple("counter", "value", *v as f64),
            };
            Some(vec![row])
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_timescaledb::Cmp::new(config_timescaledb))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_timescaledb"))]
fn main() {}
