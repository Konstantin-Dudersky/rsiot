mod shared;

use std::time::Duration;

use leptos::task::{spawn_local, Executor};
use rsiot::{
    components::{cmp_inject_periodic, cmp_logger, cmp_websocket_client_wasm},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::{LogConfig, LogConfigFilter},
    message::*,
    serde_utils::SerdeAlgKind,
};
use tokio::task::LocalSet;
use tracing::Level;

use shared::{ClientMessages, ClientToServer, ServerToClient};

fn main() -> anyhow::Result<()> {
    LogConfig {
        filter: LogConfigFilter::String("debug"),
    }
    .run()?;

    // cmp_logger ----------------------------------------------------------------------------------
    let config_logger = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                ClientMessages::ServerCounter(counter) => format!("Server counter: {}", counter),
                ClientMessages::ConnectionState(state) => format!("Connection state: {}", state),
                _ => return Ok(None),
            };
            Ok(Some(text))
        },
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter_client = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_millis(100),
        fn_periodic: move || {
            let msg = ClientMessages::CounterFromClient(counter_client);
            counter_client = counter_client.wrapping_add(1);
            vec![msg]
        },
    };

    // cmp_websocket_client_wasm -------------------------------------------------------------------
    let config_websocket_client_wasm =
        cmp_websocket_client_wasm::Config::<ClientMessages, ServerToClient, ClientToServer> {
            url: "ws://localhost:8011".to_string(),
            serde_alg: SerdeAlgKind::Json,
            fn_client_to_server: |msg| {
                let c2s = match msg {
                    ClientMessages::CounterFromClient(counter) => {
                        ClientToServer::ClientCounter(*counter)
                    }
                    _ => return None,
                };
                Some(c2s)
            },
            fn_server_to_client: |s2c: ServerToClient| {
                let msg = match s2c {
                    ServerToClient::ServerCounter(counter) => {
                        ClientMessages::ServerCounter(counter)
                    }
                };
                vec![msg]
            },
            fn_connection_state: |state| {
                Some(Message::new_custom(ClientMessages::ConnectionState(state)))
            },
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
        ComponentExecutor::<ClientMessages>::new(config_executor)
            .add_cmp(cmp_websocket_client_wasm::Cmp::new(
                config_websocket_client_wasm,
            ))
            .add_cmp(cmp_logger::Cmp::new(config_logger))
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            .wait_result()
            .await?;
        Ok(()) as anyhow::Result<()>
    });
    spawn_local(context);

    Err(anyhow::Error::msg("Program stop"))
}
