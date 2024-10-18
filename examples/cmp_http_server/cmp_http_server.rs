//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_http_server --target x86_64-unknown-linux-gnu --features cmp_http_server
//! cargo run --example cmp_http_server --target x86_64-unknown-linux-gnu --features cmp_http_server, single-thread
//!
//! Можно задать сообщение:
//!
//! ```json
//! {"MessageSet":{"value":24.0,"ts":"2024-02-12T18:57:16.717277474Z","source":null}}
//! ```

#[cfg(feature = "cmp_http_server")]
fn main() -> anyhow::Result<()> {
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "single-thread")]
    use tokio::task::LocalSet;
    use tokio::{runtime, time::Duration};
    use tracing::Level;
    use tracing_subscriber::filter::LevelFilter;

    use rsiot::{
        components::{
            cmp_http_server::{self, ConfigCmpPlcData},
            cmp_inject_periodic, cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::Service, Message, MsgDataBound},
    };

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    enum Data {
        Msg0(f64),
        Msg1(f64),
        MsgSet(f64),
    }

    impl MsgDataBound for Data {
        type TService = Service;
    }

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let mut counter = 0.0;

    let logger_config = cmp_logger::Config::<Data> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let http_server_config = cmp_http_server::Config {
        port: 8011,
        fn_output: |text: &str| {
            let msg = Message::<Data>::deserialize(text)?;
            Ok(Some(msg))
        },
        fn_input: |msg: &Message<Data>| {
            let text = msg.serialize()?;
            Ok(Some(text))
        },
        cmp_plc: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return ConfigCmpPlcData::NoData;
            };
            match msg {
                Data::Msg0(data) => ConfigCmpPlcData::Input(data.to_string()),
                Data::Msg1(data) => ConfigCmpPlcData::Output(data.to_string()),
                Data::MsgSet(_) => ConfigCmpPlcData::NoData,
            }
        },
    };

    let inject_periodic_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg1 = Message::new_custom(Data::Msg0(counter));
            let msg2 = Message::new_custom(Data::Msg1(counter * 2.0));
            counter += 1.0;
            vec![msg1, msg2]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    #[cfg(not(feature = "single-thread"))]
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            ComponentExecutor::new(executor_config)
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
        .build()?
        .block_on(async move {
            let local_set = LocalSet::new();

            local_set.spawn_local(async move {
                ComponentExecutor::new(executor_config)
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

#[cfg(not(feature = "cmp_http_server"))]
fn main() {
    unimplemented!("Features not active")
}
