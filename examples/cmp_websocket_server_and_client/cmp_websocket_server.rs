//! Простеший пример сервера websocket
//!
//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_websocket_server --features "cmp_websocket_server"
//! ```

mod shared;

#[cfg(feature = "cmp_websocket_server")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;
    use tracing::Level;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_logger, cmp_websocket_server},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::Service, *},
    };

    use shared::{ClientToServer, ServerMessages, ServerToClient};

    tracing_subscriber::fmt()
        // .with_env_filter("trace,tokio_tungstenite=debug,tungstenite=debug")
        .with_env_filter("info")
        .init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = if let ServerMessages::CounterFromClient(msg) = msg {
                format!("Counter from client: {}", msg)
            } else {
                return Ok(None);
            };
            Ok(Some(text))
        },
    };

    let ws_server_config = cmp_websocket_server::Config {
        port: 8011,
        fn_input: |msg: &Message<ServerMessages>| {
            let msg = msg.get_custom_data()?;
            let s2c = match msg {
                ServerMessages::ServerCounter(counter) => ServerToClient::ServerCounter(counter),
                _ => return None,
            };
            Some(s2c)
        },
        fn_output: |c2s: ClientToServer| {
            let msg = match c2s {
                ClientToServer::ClientCounter(counter) => {
                    Message::new_custom(ServerMessages::CounterFromClient(counter))
                }
                _ => return vec![],
            };
            vec![msg]
        },
    };

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_millis(1000),
        fn_periodic: move || {
            let msg = Message::new_custom(ServerMessages::ServerCounter(counter));
            counter += 1;
            vec![msg]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_websocket_server::Cmp::new(ws_server_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_websocket_server"))]
fn main() {}
