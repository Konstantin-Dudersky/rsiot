//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-http-server --example http-server-example
//! cargo run -p rsiot-http-server --example http-server-example --features single-thread
//! ```

use serde::{Deserialize, Serialize};
use tokio::{runtime, task::LocalSet, time::Duration};
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;

use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_http_server::cmp_http_server;
use rsiot_messages_core::IMessage;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    Combine(f64, f64),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let mut counter = 0.0;

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let http_server_config = cmp_http_server::Config { port: 8011 };

    let inject_periodic_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg1 = Message::Message0(counter);
            let msg2 = Message::Message1(counter * 2.0);
            counter += 1.0;
            vec![msg1, msg2]
        },
    };

    #[cfg(not(feature = "single-thread"))]
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            ComponentExecutor::new(100)
                .add_cmp(cmp_logger::Cmp::new(logger_config))
                .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
                .add_cmp(cmp_http_server::Cmp::new(http_server_config))
                .wait_result()
                .await?;

            Ok(()) as anyhow::Result<()>
        })?;

    #[cfg(feature = "single-thread")]
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let local_set = LocalSet::new();

            local_set.spawn_local(async move {
                ComponentExecutor::new(100)
                    .add_cmp(cmp_logger::Cmp::new(logger_config))
                    .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
                    .add_cmp(cmp_http_server::Cmp::new(http_server_config))
                    .wait_result()
                    .await?;
                Ok(()) as anyhow::Result<()>
            });

            local_set.await;

            Ok(()) as anyhow::Result<()>
        })?;

    Ok(())
}
