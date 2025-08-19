//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_http_client --features="cmp_http_client, serde_json"
//! ```

mod shared;

#[cfg(feature = "cmp_http_client")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use serde::{Deserialize, Serialize};
    use tokio::time::Duration;
    use tracing::{level_filters::LevelFilter, Level};

    use shared::{ClientToServer, ServerToClient};

    use rsiot::{
        components::{cmp_http_client, cmp_inject_periodic, cmp_logger},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{MsgDataBound, MsgKey},
        serde_utils::SerdeAlgKind,
    };

    // Message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    enum Data {
        CounterFromServer(f64),
        CounterFromClient(u8),
    }

    impl MsgDataBound for Data {}

    //----------------------------------------------------------------------------------------------

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_millis(1000),
        fn_periodic: move || {
            let msg = Data::CounterFromClient(counter);
            counter = counter.wrapping_add(1);
            vec![msg]
        },
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                Data::CounterFromServer(data) => format!("Counter from server: {data}"),
                _ => return Ok(None),
            };
            Ok(Some(text))
        },
    };

    let http_config = cmp_http_client::Config::<Data> {
        serde_alg: SerdeAlgKind::Json,
        // base_url: "http://192.168.71.1:8010",
        base_url: "http://localhost:8010".into(),
        timeout: Duration::from_secs(5),
        requests_input: vec![Box::new(cmp_http_client::RequestInputConfig::<
            Data,
            (),
            ClientToServer,
        > {
            serde_alg: SerdeAlgKind::Json,
            request_kind: cmp_http_client::RequestKind::Put,
            endpoint: "/enter".to_string(),
            fn_create_request: |msg| match msg {
                Data::CounterFromClient(counter) => {
                    let c2s = ClientToServer::SetCounterFromClient(*counter);
                    Some(c2s)
                }
                _ => None,
            },
            fn_process_response_success: |_| vec![],
            fn_process_response_error: Vec::new,
        })],
        requests_periodic: vec![Box::new(cmp_http_client::RequestPeriodicConfig::<
            Data,
            ServerToClient,
            (),
        > {
            serde_alg: SerdeAlgKind::Json,
            request_kind: cmp_http_client::RequestKind::Get,
            endpoint: "/data/test".to_string(),
            period: Duration::from_millis(1000),
            request_body: (),
            fn_process_response_success: |s2c| vec![Data::CounterFromServer(s2c.counter)],
            fn_process_response_error: Vec::new,
        })],
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_http_client::Cmp::new(http_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_http_client"))]
fn main() {}
