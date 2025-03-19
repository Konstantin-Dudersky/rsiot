//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_http_client --features cmp_http_client
//! ```

mod shared;

#[cfg(feature = "cmp_http_client")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;
    use tokio::time::Duration;
    use tracing::{level_filters::LevelFilter, Level};

    use shared::{ClientToServer, ServerToClient};

    use rsiot::{
        components::{
            cmp_http_client::{self},
            cmp_inject_periodic, cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::*, Message, MsgDataBound, MsgKey},
    };

    // Message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    enum Data {
        CounterFromServer(f64),
        CounterFromClient(u8),
    }

    impl MsgDataBound for Data {
        type TService = Service;
    }

    //------------------------------------------------------------------------------

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_millis(100),
        fn_periodic: move || {
            let msg = Message::new_custom(Data::CounterFromClient(counter));
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
                Data::CounterFromServer(data) => format!("Counter from server: {}", data),
                _ => return Ok(None),
            };
            Ok(Some(text))
        },
    };

    let http_config = cmp_http_client::Config::<Data> {
        connection_config: cmp_http_client::ConnectionConfig {
            base_url: "http://192.168.71.1:8010".into(),
            // base_url: "http://localhost:8010".into(),
        },
        requests_input: vec![cmp_http_client::RequestInput {
            fn_input: |msg| {
                let msg = msg.get_custom_data()?;
                match msg {
                    Data::CounterFromClient(counter) => {
                        let data = ClientToServer::SetCounterFromClient(counter);
                        let body = serde_json::to_string(&data).unwrap();

                        let param = cmp_http_client::HttpParam::Put {
                            endpoint: "/enter".to_string(),
                            body,
                        };
                        Some(param)
                    }
                    _ => None,
                }
            },
            on_success: |_| Ok(vec![]),
            on_failure: Vec::new,
        }],
        requests_periodic: vec![cmp_http_client::RequestPeriodic {
            period: Duration::from_millis(100),
            http_param: cmp_http_client::HttpParam::Get {
                endpoint: "/data/test".to_string(),
            },
            on_success: |body| {
                let res = from_str::<ServerToClient>(body)?;
                Ok(vec![Message::new_custom(Data::CounterFromServer(
                    res.counter,
                ))])
            },
            on_failure: Vec::new,
        }],
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
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
