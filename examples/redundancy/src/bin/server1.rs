use tokio::main;

use rsiot::{
    components::{cmp_redis_client, cmp_websocket_server},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::{example_message::*, *},
};
use url::Url;

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "server1".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let ws_server_config = cmp_websocket_server::Config {
        port: 8010,
        fn_input: |msg: &Message<Custom>| {
            let text = msg.serialize()?;
            Ok(Some(text))
        },
        fn_output: |text: &str| {
            let msg = Message::<Custom>::deserialize(text)?;
            Ok::<Option<Vec<Message<Custom>>>, anyhow::Error>(Some(vec![msg]))
        },
    };

    let redis_config = cmp_redis_client::Config {
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

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_websocket_server::Cmp::new(ws_server_config))
        .add_cmp(cmp_redis_client::Cmp::new(redis_config))
        .wait_result()
        .await
        .unwrap()
}
