use std::time::Duration;

use rsiot::{
    components::cmp_logger,
    executor::{ComponentExecutor, ComponentExecutorConfig},
};
use tracing::Level;

use crate::{config_mqtt_server_subscribe, message::Custom};

pub async fn subscribe() {
    let config_logger = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                Custom::Counter(c) => format!("{c}"),
            };
            Ok(Some(text))
        },
    };

    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::<Custom>::new(config_executor)
        .add_cmp(cmp_logger::Cmp::new(config_logger))
        .add_cmp(config_mqtt_server_subscribe::cmp())
        .wait_result()
        .await
        .unwrap();
}
