//! Простеший пример сервера websocket
//!
//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot --example cmp_websocket_server --features "cmp_websocket_server"
//! ```

#[cfg(feature = "cmp_websocket_server")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;
    use tracing::Level;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_logger, cmp_websocket_server},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, example_service::Service, *},
    };

    tracing_subscriber::fmt()
        .with_env_filter("trace,tokio_tungstenite=debug,tungstenite=debug")
        .init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let ws_server_config = cmp_websocket_server::Config {
        port: 8021,
        fn_input: |msg: &Message<Custom>| {
            let text = msg.serialize()?;
            Ok(Some(text))
        },
        fn_output: |text: &str| {
            let msg = Message::deserialize(text)?;
            Ok(Some(vec![msg]))
        },
    };

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(10),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::ValueInstantF64(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
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
