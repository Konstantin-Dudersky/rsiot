//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_http_server --features="cmp_http_server, serde_json"
//! cargo run --example cmp_http_server --target x86_64-unknown-linux-gnu --features cmp_http_server, single-thread
//!

#[cfg(feature = "cmp_http_server")]
mod shared;

#[cfg(feature = "cmp_http_server")]
fn main() -> anyhow::Result<()> {
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "single-thread")]
    use tokio::task::LocalSet;
    use tokio::{runtime, time::Duration};
    use tracing::Level;
    use tracing_subscriber::filter::LevelFilter;

    use shared::{ClientToServer, ServerToClient};

    use rsiot::{
        components::{
            cmp_http_server::{self, GetEndpointConfig},
            cmp_inject_periodic, cmp_logger,
        },
        components_config::http_server::PutEndpointConfig,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{Message, MsgDataBound, MsgKey},
        serde_utils::SerdeAlgKind,
    };

    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    enum Data {
        Counter(f64),
        Msg1(f64),
        CounterFromClient(u8),
    }

    impl MsgDataBound for Data {}

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let mut counter = 0.0;

    let logger_config = cmp_logger::Config::<Data> {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                Data::CounterFromClient(data) => format!("Counter from client: {}", data),
                _ => return Ok(None),
            };

            Ok(Some(text))
        },
    };

    let http_server_config = cmp_http_server::Config {
        port: 8010,
        get_endpoints: vec![Box::new(GetEndpointConfig {
            serde_alg: SerdeAlgKind::Json,
            path: "/data/test",
            server_to_client_data: ServerToClient::default(),
            fn_input: |msg, data| {
                if let Data::Counter(counter) = msg {
                    data.counter = *counter
                }
            },
        })],
        put_endpoints: vec![Box::new(PutEndpointConfig {
            serde_alg: SerdeAlgKind::Json,
            path: "/enter",
            fn_output: |data: ClientToServer| match data {
                ClientToServer::NoData => None,
                ClientToServer::SetCounterFromClient(data) => {
                    Some(Message::new_custom(Data::CounterFromClient(data)))
                }
            },
        })],
    };

    let inject_periodic_config = cmp_inject_periodic::Config {
        period: Duration::from_millis(100),
        fn_periodic: move || {
            let msg1 = Data::Counter(counter);
            let msg2 = Data::Msg1(counter * 2.0);
            counter += 1.0;
            vec![msg1, msg2]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
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
