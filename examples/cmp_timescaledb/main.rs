#[cfg(feature = "cmp_timescaledb")]
mod config_timescaledb;
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
        components::cmp_inject_periodic,
        executor::{ComponentExecutor, ComponentExecutorConfig},
    };

    use message::Msg;

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_millis(10),
        fn_periodic: move || {
            let msg = Msg::Counter(counter);
            counter += 1;
            vec![msg]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(config_timescaledb::cmp())
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_timescaledb"))]
fn main() {}
