//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot --example cmp_http_client --features "cmp_http_client"
//! ```

#[cfg(feature = "cmp_http_client")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use serde_json::from_str;
    use tokio::time::Duration;
    use tracing::{level_filters::LevelFilter, Level};

    use rsiot::{
        components::{
            cmp_http_client::{self},
            cmp_inject_periodic, cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::*, Message, MsgDataBound},
    };

    //------------------------------------------------------------------------------

    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    enum Data {
        HttpMethodsGetPeriodicRespone(HttpMethodsGet),
        HttpMethodsGetOnEventResponse(HttpMethodsGet),
        HttpMethodsGetOnEventRequest(()),
    }

    impl MsgDataBound for Data {
        type TService = Service;
    }

    //------------------------------------------------------------------------------

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    struct HttpMethodsGet {
        args: HashMap<String, String>,
        headers: HashMap<String, String>,
        origin: String,
        url: String,
    }

    //------------------------------------------------------------------------------

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Data::HttpMethodsGetOnEventRequest(()));
            vec![msg]
        },
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let http_config = cmp_http_client::Config::<Data> {
        connection_config: cmp_http_client::ConnectionConfig {
            base_url: "http://127.0.0.1:80".into(),
        },
        requests_input: vec![cmp_http_client::RequestInput {
            fn_input: |msg| {
                let msg = msg.get_custom_data()?;
                match msg {
                    Data::HttpMethodsGetOnEventRequest(_) => {
                        let param = cmp_http_client::HttpParam::Get {
                            endpoint: "get".to_string(),
                        };
                        Some(param)
                    }
                    _ => None,
                }
            },
            on_success: |body| {
                let res = from_str::<HttpMethodsGet>(body)?;
                Ok(vec![Message::new_custom(
                    Data::HttpMethodsGetOnEventResponse(res),
                )])
            },
            on_failure: Vec::new,
        }],
        requests_periodic: vec![cmp_http_client::RequestPeriodic {
            period: Duration::from_secs(5),
            http_param: cmp_http_client::HttpParam::Get {
                endpoint: "get".to_string(),
            },
            on_success: |body| {
                let res = from_str::<HttpMethodsGet>(body)?;
                Ok(vec![Message::new_custom(
                    Data::HttpMethodsGetPeriodicRespone(res),
                )])
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
