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

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", feature = "single-thread"),
    all(target_arch = "riscv32", feature = "single-thread"),
))]
fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use tokio::runtime;
    #[cfg(feature = "single-thread")]
    use tokio::task::LocalSet;

    use rsiot_component_core::ComponentExecutor;
    use rsiot_extra_components::{
        cmp_derive::{self, DeriveItem},
        cmp_inject_periodic, cmp_logger,
    };
    use rsiot_messages_core::{example_message::*, Message};
    use tracing::Level;

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
                Custom::ValueInstantF64(content) => store.f64 = Some(*content),
                Custom::ValueInstantBool(content) => store.bool = Some(*content),
                _ => (),
            },
            fn_output: |value| {
                let msg_content =
                    format!("New Message: bool: {}, f64: {}", value.bool?, value.f64?);
                let msg = Custom::ValueInstantString(msg_content);
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
            let msg1 = Message::new(Custom::ValueInstantF64(counter));
            let msg2 = Message::new(Custom::ValueInstantBool(true));
            counter += 1.0;
            vec![msg1, msg2]
        },
    };

    #[cfg(not(feature = "single-thread"))]
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            ComponentExecutor::new(100, "cmp_derive")
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
                ComponentExecutor::new(100, "cmp_derive")
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

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", feature = "single-thread"),
    all(target_arch = "riscv32", feature = "single-thread"),
)))]
fn main() {}
