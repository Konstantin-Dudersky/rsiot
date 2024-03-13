//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_redis_client_publication --features "cmp_redis_client" --target="x86_64-unknown-linux-gnu"
//! ```

#[cfg(feature = "cmp_redis_client")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;
    use tracing::Level;
    use tracing_subscriber::fmt;
    use url::Url;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_logger, cmp_redis_client},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, *},
    };

    fmt().init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let redis_config = cmp_redis_client::Config {
        url: Url::parse("redis://127.0.0.1:6379")?,
        subscription_channel: ExampleMessageChannel::Output,
        fn_input: |msg: &Message<Custom>| {
            let channel = ExampleMessageChannel::Output;
            let key = msg.key.clone();
            let value = msg.serialize()?;
            Ok(Some(vec![cmp_redis_client::ConfigFnInputItem {
                channel,
                key,
                value,
            }]))
        },
        fn_output: |text: &str| {
            let msg = Message::deserialize(text)?;
            Ok(Some(vec![msg]))
        },
    };

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::ValueInstantF64(counter as f64));

            counter += 1;
            vec![msg]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "redis-client-publication".into(),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_redis_client::Cmp::new(redis_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_redis_client"))]
fn main() {}
