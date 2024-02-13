//! Простеший пример сервера websocket
//!
//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-websocket-server --example ws_server_example
//! ```

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;
    use tracing::Level;

    use rsiot_component_core::ComponentExecutor;
    use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
    use rsiot_messages_core::{ExampleMessage, IMessage, MsgContent};
    use rsiot_websocket_server::cmp_websocket_server;

    tracing_subscriber::fmt()
        .with_env_filter("trace,tokio_tungstenite=debug,tungstenite=debug")
        .init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let ws_server_config = cmp_websocket_server::Config {
        port: 8021,
        fn_input: |msg: &ExampleMessage| {
            let text = msg.to_json()?;
            Ok(Some(text))
        },
        fn_output: |data: &str| {
            let msg = ExampleMessage::from_json(data)?;
            Ok(Some(msg))
        },
    };

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(10),
        fn_periodic: move || {
            let msg = ExampleMessage::ValueInstantF64(MsgContent::new(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    ComponentExecutor::new(100)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_websocket_server::Cmp::new(ws_server_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
