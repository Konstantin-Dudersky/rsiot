use std::time::Duration;

use tokio::main;

use rsiot::{
    components::{cmp_inject_periodic, cmp_redis_client},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::{example_message::*, *},
};
use url::Url;

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "device".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let redis1_config = cmp_redis_client::Config {
        url: Url::parse("redis://localhost:8012").unwrap(),
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

    let redis2_config = cmp_redis_client::Config {
        url: Url::parse("redis://localhost:8014").unwrap(),
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

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::ValueInstantF64(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_redis_client::Cmp::new(redis1_config))
        .add_cmp(cmp_redis_client::Cmp::new(redis2_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .wait_result()
        .await
        .unwrap()
}
