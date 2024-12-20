#[cfg(feature = "cmp_influxdb")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use tracing::level_filters::LevelFilter;

    use rsiot::{
        components::{cmp_influxdb, cmp_inject_periodic},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, example_service::*, *},
    };

    const TOKEN: &str =
        "6ux3LH1s0wOf4z2vIec6cmYYk03GgTksvxD3OnaM71xfOfyj9NQTvKq8TZRb5iInEl_PpoVFHFQB43CyaoJMhg==";

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

    let influxdb_config = cmp_influxdb::Config {
        host: "localhost".into(),
        port: 8086,
        org: "test".into(),
        bucket: "test1".into(),
        token: TOKEN.into(),
        fn_input: |msg: &Message<Custom>| {
            let value = match &msg.data {
                MsgData::Custom(Custom::ValueInstantF64(data)) => {
                    cmp_influxdb::ValueType::f64(*data)
                }
                _ => return None,
            };
            let line = cmp_influxdb::LineProtocolItem::new(&msg.key, value, &msg.ts);
            Some(vec![line])
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_influxdb::Cmp::new(influxdb_config))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_influxdb"))]
fn main() {}
