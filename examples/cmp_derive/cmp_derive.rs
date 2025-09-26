//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_derive --features "executor, log_tokio"
//! ```
//!
//!
//! ```bash
//! cargo run --package rsiot-extra-components --example cmp_derive --features single-thread
//! ```

#[cfg(feature = "executor")]
fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use tokio::runtime;
    #[cfg(feature = "single-thread")]
    use tokio::task::LocalSet;
    use tracing::Level;

    use rsiot::{
        components::{
            cmp_derive::{self, DeriveItem},
            cmp_inject_periodic, cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::example_message::*,
    };

    tracing_subscriber::fmt().init();

    #[derive(Clone, Default, PartialEq)]
    struct ValueInstantString {
        pub f64: Option<f64>,
        pub bool: Option<bool>,
    }

    let derive_config = cmp_derive::Config::<Custom> {
        derive_items: vec![Box::new(DeriveItem {
            store: ValueInstantString::default(),
            fn_input: |msg: &Custom, store| match msg {
                Custom::ValueInstantF64(content) => store.f64 = Some(*content),
                Custom::ValueInstantBool(content) => store.bool = Some(*content),
                _ => (),
            },
            fn_output: |store| {
                let msg_content =
                    format!("New Message: bool: {}, f64: {}", store.bool?, store.f64?);
                let msg = Custom::ValueInstantString(msg_content);
                Some(vec![msg])
            },
        })],
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Custom::ValueInstantString(content) => content,
                _ => return Ok(None),
            };

            Ok(Some(text))
        },
    };

    let mut counter = 0.0;
    let inject_periodic_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg1 = Custom::ValueInstantF64(counter);
            let msg2 = Custom::ValueInstantBool(true);
            counter += 1.0;
            vec![msg1, msg2]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_secs(0),
        fn_tokio_metrics: |_| None,
    };

    #[cfg(not(feature = "single-thread"))]
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            ComponentExecutor::new(executor_config)
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
                ComponentExecutor::new(executor_config)
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

    Err(anyhow::Error::msg("Stop execution"))
}

#[cfg(not(feature = "executor"))]
fn main() {
    unimplemented!()
}
