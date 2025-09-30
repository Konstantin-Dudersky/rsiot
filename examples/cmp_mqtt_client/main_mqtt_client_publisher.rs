#[cfg(feature = "cmp_mqtt_client")]
mod config_inject_periodic;
#[cfg(feature = "cmp_mqtt_client")]
mod config_mqtt_client;
#[cfg(feature = "cmp_mqtt_client")]
mod message;

#[cfg(feature = "cmp_mqtt_client")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    tracing_subscriber::fmt().init();

    let config_executor = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(10),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(config_executor)
        .add_cmp(config_mqtt_client::publisher::cmp())
        .add_cmp(config_inject_periodic::cmp())
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_mqtt_client"))]
fn main() {}
