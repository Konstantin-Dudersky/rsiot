use std::time::Duration;

use rsiot::{
    components::{cmp_inject_periodic, cmp_mqtt_client},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::Message,
};

use super::message;

pub async fn publish() {
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "publish".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let mut counter = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(message::Custom::Counter(counter));
            counter += 1;
            vec![msg]
        },
    };

    let config_mqtt_client = cmp_mqtt_client::Config {
        client_id: "pubish".into(),
        host: "localhost".into(),
        port: 1883,
        fn_input: |msg| Ok(Some(msg.serialize()?.into_bytes())),
        fn_output: |_| Ok(None),
    };

    ComponentExecutor::<message::Custom>::new(config_executor)
        .add_cmp(cmp_mqtt_client::Cmp::new(config_mqtt_client))
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .wait_result()
        .await
        .unwrap();
}
