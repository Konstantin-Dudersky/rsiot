//! Запуск:
//!
//! ```bash
//! cargo run --target x86_64-unknown-linux-gnu --example cmp_telegram --features cmp_telegram
//! ```

#[cfg(feature = "cmp_telegram")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_telegram},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, *},
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

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "timescaledb-storing".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let config_telegram = cmp_telegram::Config {
        fn_input: |_| None,
        fn_output: |_| vec![],
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_telegram::Cmp::new(config_telegram))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_telegram"))]
fn main() {
    unimplemented!()
}
