//! Запуск:
//!
//! ```bash
//! cargo run --package rsiot-extra-components --example cmp_derive
//! ```
//!
//!
//! ```bash
//! cargo run --package rsiot-extra-components --example cmp_derive --features single-thread
//! ```

use std::time::Duration;

use tokio::runtime;
#[cfg(feature = "single-thread")]
use tokio::task::LocalSet;

use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::{
    cmp_derive::{self, DeriveItem},
    cmp_inject_periodic, cmp_logger,
};
use rsiot_messages_core::{ExampleMessage, MsgContent};
use tracing::Level;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    #[derive(Clone, Default, PartialEq)]
    struct ValueInstantString {
        pub f64: Option<f64>,
        pub bool: Option<bool>,
    }

    let derive_config = cmp_derive::Config {
        derive_items: vec![Box::new(DeriveItem {
            store: ValueInstantString::default(),
            fn_input: |msg, store| match msg {
                ExampleMessage::ValueInstantF64(content) => store.f64 = Some(content.value),
                ExampleMessage::ValueInstantBool(content) => store.bool = Some(content.value),
                _ => (),
            },
            fn_output: |value| {
                let msg_content =
                    format!("New Message: bool: {}, f64: {}", value.bool?, value.f64?);
                let msg = ExampleMessage::ValueInstantString(MsgContent::new(msg_content));
                Some(vec![msg])
            },
        })],
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let mut counter = 0.0;
    let inject_periodic_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg1 = ExampleMessage::ValueInstantF64(MsgContent::new(counter));
            let msg2 = ExampleMessage::ValueInstantBool(MsgContent::new(true));
            counter += 1.0;
            vec![msg1, msg2]
        },
    };

    #[cfg(not(feature = "single-thread"))]
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            ComponentExecutor::new(100)
                .add_cmp(cmp_derive::Cmp::new(derive_config))
                .add_cmp(cmp_logger::Cmp::new(logger_config))
                .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
                .wait_result()
                .await?;
            Ok(()) as anyhow::Result<()>
        })?;

    #[cfg(feature = "single-thread")]
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let local_set = LocalSet::new();

            local_set.spawn_local(async move {
                ComponentExecutor::new(100)
                    .add_cmp(cmp_derive::Cmp::new(derive_config))
                    .add_cmp(cmp_logger::Cmp::new(logger_config))
                    .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
                    .wait_result()
                    .await?;
                Ok(()) as anyhow::Result<()>
            });

            local_set.await;

            Ok(()) as anyhow::Result<()>
        })?;

    Ok(())
}
