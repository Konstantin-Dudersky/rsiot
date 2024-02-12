#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot_component_core::ComponentExecutor;
    use rsiot_extra_components::cmp_inject_periodic;
    use rsiot_influxdb as cmp_influxdb;
    use rsiot_messages_core::{ExampleMessage, MsgContent};
    use tracing::level_filters::LevelFilter;

    const TOKEN: &str =
        "6ux3LH1s0wOf4z2vIec6cmYYk03GgTksvxD3OnaM71xfOfyj9NQTvKq8TZRb5iInEl_PpoVFHFQB43CyaoJMhg==";

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = ExampleMessage::ValueInstantF64(MsgContent::new(counter as f64));
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
        fn_input: |msg: &ExampleMessage| cmp_influxdb::msg_into_line_protocol(msg),
    };

    ComponentExecutor::new(100)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_influxdb::Cmp::new(influxdb_config))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
