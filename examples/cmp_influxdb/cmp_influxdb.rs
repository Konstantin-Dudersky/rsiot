#[cfg(feature = "cmp_influxdb")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use tracing::level_filters::LevelFilter;

    use rsiot::{
        components::{cmp_influxdb3, cmp_inject_periodic},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, *},
    };

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::ValueInstantF64(counter as f64));
            counter += 1;
            vec![msg]
        },
    };

    let influxdb_config = cmp_influxdb3::Config {
        host: "localhost".into(),
        port: 8086,
        database: "test1".into(),
        send_period: Duration::from_secs(1),
        fn_input: |msg: &Message<Custom>| {
            let value = match &msg.data {
                MsgData::Custom(Custom::ValueInstantF64(data)) => *data,
                _ => return None,
            };
            let line = cmp_influxdb3::LineProtocolItem::new_simple(&msg.key, value);
            Some(vec![line])
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_influxdb3::Cmp::new(influxdb_config))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_influxdb"))]
fn main() {}
