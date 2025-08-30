mod shared;

use std::time::Duration;

use leptos::task::{spawn_local, Executor};
use rsiot::{
    components::{cmp_http_client_wasm, cmp_inject_periodic, cmp_logger},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::{LogConfig, LogConfigFilter},
    message::*,
    serde_utils::SerdeAlgKind,
};
use tokio::task::LocalSet;
use tracing::Level;

use shared::{ClientToServer, ServerToClient};

fn main() -> anyhow::Result<()> {
    LogConfig {
        filter: LogConfigFilter::String("debug"),
    }
    .run()?;

    // Message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    enum Data {
        CounterFromServer(f64),
        CounterFromClient(u8),
    }

    impl MsgDataBound for Data {}

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                Data::CounterFromServer(data) => format!("Counter from server: {}", data),
                _ => return Ok(None),
            };
            Ok(Some(text))
        },
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_millis(1000),
        fn_periodic: move || {
            let msg = Data::CounterFromClient(counter);
            counter = counter.wrapping_add(1);
            vec![msg]
        },
    };

    // cmp_websocket_client_wasm -------------------------------------------------------------------
    let http_config = cmp_http_client_wasm::Config::<Data> {
        serde_alg: SerdeAlgKind::Json,
        // base_url: "http://192.168.71.1:8010",
        base_url: "http://localhost:8010".into(),
        timeout: Duration::from_secs(5),
        requests_input: vec![Box::new(cmp_http_client_wasm::RequestInputConfig::<
            Data,
            (),
            ClientToServer,
        > {
            serde_alg: SerdeAlgKind::Json,
            request_kind: cmp_http_client_wasm::RequestKind::Put,
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
        requests_periodic: vec![Box::new(cmp_http_client_wasm::RequestPeriodicConfig::<
            Data,
            ServerToClient,
            (),
        > {
            serde_alg: SerdeAlgKind::Json,
            request_kind: cmp_http_client_wasm::RequestKind::Get,
            endpoint: "/data/test".to_string(),
            period: Duration::from_millis(500),
            request_body: (),
            fn_process_response_success: |s2c| vec![Data::CounterFromServer(s2c.counter)],
            fn_process_response_error: Vec::new,
        })],
    };

    // executor ------------------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(200),
        fn_tokio_metrics: |_| None,
    };

    Executor::init_wasm_bindgen().expect("executor should only be initialized once");

    let context = LocalSet::new();

    context.spawn_local(async move {});

    context.spawn_local(async move {
        ComponentExecutor::<Data>::new(config_executor)
            .add_cmp(cmp_http_client_wasm::Cmp::new(http_config))
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
            .wait_result()
            .await?;
        Ok(()) as anyhow::Result<()>
    });
    spawn_local(context);

    Err(anyhow::Error::msg("Program end"))
}
