use std::time::Duration;

use rsiot::{
    components::cmp_inject_periodic,
    executor::{ComponentExecutor, ComponentExecutorConfig},
};

use crate::{config_mqtt_server_publish, message::Custom};

pub async fn publish() {
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let mut counter = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_millis(100),
        fn_periodic: move || {
            let msg = Custom::Counter(counter);
            counter += 1;
            vec![msg]
        },
    };

    ComponentExecutor::new(config_executor)
        .add_cmp(config_mqtt_server_publish::cmp())
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .wait_result()
        .await
        .unwrap();
}
