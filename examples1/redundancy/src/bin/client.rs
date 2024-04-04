use tokio::main;

use rsiot::{
    components::{cmp_logger, cmp_websocket_client},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::{example_message::*, *},
};
use tracing::Level;
use url::Url;

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "device".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let ws_client_1 = cmp_websocket_client::Config {
        url: Url::parse("ws://localhost:8010").unwrap(),
        fn_input: |msg: &Message<Custom>| {
            let text = msg.serialize()?;
            Ok(Some(text))
        },
        fn_output: |text: &str| {
            let msg = Message::deserialize(text)?;
            Ok(Some(vec![msg]))
        },
    };

    let ws_client_2 = cmp_websocket_client::Config {
        url: Url::parse("ws://localhost:8011").unwrap(),
        fn_input: |msg: &Message<Custom>| {
            let text = msg.serialize()?;
            Ok(Some(text))
        },
        fn_output: |text: &str| {
            let msg = Message::deserialize(text)?;
            Ok(Some(vec![msg]))
        },
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_websocket_client::Cmp::new(ws_client_1))
        .add_cmp(cmp_websocket_client::Cmp::new(ws_client_2))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .wait_result()
        .await
        .unwrap()
}
