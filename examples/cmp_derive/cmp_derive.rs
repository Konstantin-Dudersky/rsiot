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

#[cfg(all(feature = "executor", feature = "log_console", feature = "log_tokio"))]
fn main() -> anyhow::Result<()> {
    use std::{
        net::{Ipv4Addr, SocketAddrV4},
        time::Duration,
    };

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
        logging::{LogConfig, LogConfigFilter},
        message::{example_message::*, *},
    };

    LogConfig {
        filter: LogConfigFilter::String("info"),
        tokio_console_addr: SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 6669),
        loki_url: String::from("http://service_loki:3100"),
    }
    .run()
    .unwrap();

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

    Ok(())
}

#[cfg(not(all(feature = "executor", feature = "log_console", feature = "log_tokio")))]
fn main() {
    unimplemented!()
}
