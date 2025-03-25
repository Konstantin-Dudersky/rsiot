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
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let config_telegram = cmp_telegram::Config {
        bot_token: "7010894920:AAFMdSlQ6d3Jvosa5DGWitcI3Dpm0ZKGXj4".into(),
        chat_id: -1002220119164,
        fn_input: |msg| match msg.get_custom_data()? {
            Custom::ValueInstantF64(counter) => Some(format!("Counter: {}", counter)),
            _ => None,
        },
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
