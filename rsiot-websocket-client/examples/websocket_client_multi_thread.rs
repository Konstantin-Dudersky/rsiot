//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-websocket-client --example websocket_client_multi_thread
//! ```

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use serde::{Deserialize, Serialize};
    use tokio::time::Duration;
    use tracing::Level;
    use url::Url;

    use rsiot_component_core::{ComponentExecutor, ComponentExecutorConfig};
    use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
    use rsiot_messages_core::{Message, MsgDataBound};
    use rsiot_websocket_client::cmp_websocket_client;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    enum Data {
        Send(f64),
        Recv(f64),
        Tick(u64),
    }

    impl MsgDataBound for Data {}

    fn fn_input(msg: &Message<Data>) -> anyhow::Result<Option<String>> {
        let text = msg.serialize()?;
        Ok(Some(text))
    }

    fn fn_output(text: &str) -> anyhow::Result<Option<Vec<Message<Data>>>> {
        // сообщение tick ...
        if let Some(val) = parse_tick(text) {
            return Ok(Some(vec![val]));
        }
        let msg = Message::deserialize(text)?;
        let msg = match msg.get_data() {
            Some(msg) => msg,
            None => return Ok(None),
        };
        match msg {
            Data::Send(val) => Ok(Some(vec![Message::new_custom(Data::Recv(val))])),
            Data::Recv(_) => Ok(None),
            Data::Tick(_) => Ok(None),
        }
    }

    fn parse_tick(data: &str) -> Option<Message<Data>> {
        let parts: Vec<&str> = data.split(' ').collect();
        if parts.len() != 2 {
            return None;
        }
        if parts[0] != "tick" {
            return None;
        }
        let num: Option<u64> = parts[1].parse().ok();
        let num = match num {
            Some(val) => val,
            None => return None,
        };
        Some(Message::new_custom(Data::Tick(num)))
    }

    tracing_subscriber::fmt().init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Data::Send(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    let ws_client = cmp_websocket_client::Config {
        url: Url::parse("ws://localhost:9001")?,
        fn_input,
        fn_output,
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "rsiot-websocket-client".into(),
        fn_auth: |_| None,
    };

    ComponentExecutor::<Data>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_websocket_client::Cmp::new(ws_client))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
