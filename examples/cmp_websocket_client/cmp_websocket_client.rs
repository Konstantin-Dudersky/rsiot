//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_websocket_client --features "cmp_websocket_client" --target="x86_64-unknown-linux-gnu"
//! ```

#[cfg(feature = "cmp_websocket_client")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use serde::{Deserialize, Serialize};
    use tokio::time::Duration;
    use tracing::Level;

    use rsiot::{
        components::{cmp_inject_periodic, cmp_logger, cmp_websocket_client},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::Service, Message, MsgDataBound, MsgKey},
    };

    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    enum Data {
        Send(f64),
        Recv(f64),
        Tick(u64),
    }

    impl MsgDataBound for Data {
        type TService = Service;
    }

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
        let msg = match msg.get_custom_data() {
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
        fn_input: |msg| Ok(Some(msg.serialize()?)),
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
        url: "ws://localhost:9001".into(),
        fn_input,
        fn_output,
        fn_connection_state: |_| None,
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<Data, Service>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_websocket_client::Cmp::new(ws_client))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_websocket_client"))]
fn main() {}
