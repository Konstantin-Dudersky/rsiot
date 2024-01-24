//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-redis-client --example publication
//! ```

use tokio::{main, time::Duration};
use tracing::Level;
use tracing_subscriber::fmt;
use url::Url;

use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{msg_meta::ServiceId, ExampleMessage, ExampleMessageChannel, MsgContent};
use rsiot_redis_client::cmp_redis_client;

#[main]
async fn main() -> anyhow::Result<()> {
    fmt().init();

    let service_id = ServiceId::parse_str("c13064d3-9460-4e82-b96c-c4d889f706c6").unwrap();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let redis_config = cmp_redis_client::Config {
        service_id,
        url: Url::parse("redis://127.0.0.1:6379")?,
        fn_input: |_| vec![ExampleMessageChannel::Output],
        subscription_channel: ExampleMessageChannel::Output,
    };

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = ExampleMessage::ValueInstantF64(MsgContent::new(counter as f64));

            counter += 1;
            vec![msg]
        },
    };

    ComponentExecutor::new(100)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_redis_client::Cmp::new(redis_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .wait_result()
        .await?;

    Ok(())
}
