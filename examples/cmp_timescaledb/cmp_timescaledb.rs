//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-timescaledb-storing --example timescaledb-storing
//! ```

#[cfg(feature = "cmp_timescaledb")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;
    use url::Url;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_timescaledb},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, example_service::*, Message},
    };

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
    let db_config = cmp_timescaledb::Config {
        connection_string: url,
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_timescaledb::Cmp::new(db_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_timescaledb"))]
fn main() {}
