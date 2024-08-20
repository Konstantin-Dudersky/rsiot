use rsiot::{
    components::{cmp_logger, cmp_mqtt_client},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::Message,
};
use tracing::Level;

use super::message;

pub async fn subscribe() {
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        service: message::Services::subscribe,
        fn_auth: |msg, _| Some(msg),
    };

    let config_logger = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let text = msg.serialize()?;
            let text = format!("Subscribe: {text}");
            Ok(Some(text))
        },
    };

    let config_mqtt_client = cmp_mqtt_client::Config {
        client_id: "subscribe".into(),
        host: "localhost".into(),
        port: 1883,
        fn_input: |_| Ok(None),
        fn_output: |payload: &[u8]| {
            let payload = String::from_utf8_lossy(payload);
            let msg = Message::deserialize(&payload)?;
            Ok(Some(msg))
        },
    };

    ComponentExecutor::<message::Custom>::new(config_executor)
        .add_cmp(cmp_logger::Cmp::new(config_logger))
        .add_cmp(cmp_mqtt_client::Cmp::new(config_mqtt_client))
        .wait_result()
        .await
        .unwrap();
}
