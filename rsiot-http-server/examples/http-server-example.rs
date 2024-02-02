//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-http-server --example http-server-example
//! cargo run -p rsiot-http-server --example http-server-example --features single-thread
//! ```

use serde::{Deserialize, Serialize};
#[cfg(feature = "single-thread")]
use tokio::task::LocalSet;
use tokio::{runtime, time::Duration};
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;

use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_http_server::cmp_http_server;
use rsiot_messages_core::{msg_meta, IMessage, IMsgContentValue, MsgContent, MsgMeta};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
enum Message {
    Message0(MsgContent<f64>),
    Message1(MsgContent<f64>),
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
            let msg1 = Message::Message0(MsgContent::new(counter));
            let msg2 = Message::Message1(MsgContent::new(counter * 2.0));
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
