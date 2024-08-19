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
        message::{example_message::*, *},
    };

    tracing_subscriber::fmt().init();

    #[allow(non_camel_case_types)]
    #[derive(Clone, Debug)]
    enum Services {
        example_single_thread,
    }

    impl ServiceBound for Services {}

    #[derive(Clone, Default, PartialEq)]
    struct ValueInstantString {
        pub f64: Option<f64>,
        pub bool: Option<bool>,
    }

    let derive_config = cmp_derive::Config {
        derive_items: vec![Box::new(DeriveItem {
            store: ValueInstantString::default(),
            fn_input: |msg, store| match &msg.data {
                MsgData::Custom(data) => match data {
                    Custom::ValueInstantF64(content) => store.f64 = Some(*content),
                    Custom::ValueInstantBool(content) => store.bool = Some(*content),
                    _ => (),
                },
                MsgData::System(_) => (),
            },
            fn_output: |store| {
                let msg_content =
                    format!("New Message: bool: {}, f64: {}", store.bool?, store.f64?);
                let msg = Message::new(MsgData::Custom(Custom::ValueInstantString(msg_content)));
                Some(vec![msg])
            },
        })],
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let mut counter = 0.0;
    let inject_periodic_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg1 = Message::new_custom(Custom::ValueInstantF64(counter));
            let msg2 = Message::new_custom(Custom::ValueInstantBool(true));
            counter += 1.0;
            vec![msg1, msg2]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Services::example_single_thread,
        fn_auth: |msg, _| Some(msg),
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

    Ok(())
}

#[cfg(not(feature = "executor"))]
fn main() {}
