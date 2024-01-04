//! Простеший пример сервера websocket
//!
//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-websocket-server --example example1
//! ```

use tokio::{main, time::Duration};
use tracing::Level;

use rsiot_component_core::ComponentCollection;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{msg_types::Value, ExampleMessage, IMessage};
use rsiot_websocket_server::cmp_websocket_server;

#[main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let mut counter = 0.0;

    let mut chain = ComponentCollection::new(
        100,
        vec![
            cmp_inject_periodic::new(cmp_inject_periodic::Config {
                period: Duration::from_secs(10),
                fn_periodic: move || {
                    let msg = ExampleMessage::ValueInstantF64(Value::new(counter));
                    counter += 1.0;
                    vec![msg]
                },
            }),
            cmp_websocket_server::new(cmp_websocket_server::Config {
                port: 8020,
                fn_input: |msg: &ExampleMessage| msg.to_json().ok(),
                fn_output: |data: &str| ExampleMessage::from_json(data).ok(),
            }),
            cmp_logger::new(cmp_logger::Config {
                level: Level::INFO,
                header: "".into(),
            }),
        ],
    );

    chain.spawn().await?;
    Ok(())
}
