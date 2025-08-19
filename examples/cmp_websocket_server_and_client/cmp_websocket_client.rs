//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_websocket_client --features "cmp_websocket_client, serde_json"
//! ```

mod shared;

#[cfg(feature = "cmp_websocket_client")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use serde::{Deserialize, Serialize};
    use tokio::time::Duration;
    use tracing::Level;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_logger, cmp_websocket_client},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{Message, MsgDataBound, MsgKey},
        serde_utils::SerdeAlgKind,
    };

    use shared::{ClientMessages, ClientToServer, ServerToClient};

    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    enum Data {
        Send(f64),
        Recv(f64),
        Tick(u64),
    }

    impl MsgDataBound for Data {}

    tracing_subscriber::fmt().init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                ClientMessages::ServerCounter(counter) => {
                    format!("Counter from server: {counter}")
                }
                ClientMessages::ConnectionState(state) => {
                    format!("Connection state: {state:?}")
                }
                _ => return Ok(None),
            };
            Ok(Some(text))
        },
    };

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = ClientMessages::CounterFromClient(counter);
            counter = counter.wrapping_add(1);
            vec![msg]
        },
    };

    let config_websocket_client = cmp_websocket_client::Config {
        serde_alg: SerdeAlgKind::Json,
        url: "ws://localhost:8011".into(),
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
                ServerToClient::ServerCounter(counter) => ClientMessages::ServerCounter(counter),
            };
            vec![msg]
        },
        fn_connection_state: |state| {
            Some(Message::new_custom(ClientMessages::ConnectionState(state)))
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::<ClientMessages>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_websocket_client::Cmp::new(config_websocket_client))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_websocket_client"))]
fn main() {}
