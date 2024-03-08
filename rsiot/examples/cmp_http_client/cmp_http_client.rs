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
    use url::Url;

    use rsiot::{
        components::{
            cmp_http_client::{self, http_client_config},
            cmp_inject_periodic, cmp_logger,
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{Message, MsgDataBound},
    };

    //------------------------------------------------------------------------------

    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    enum Data {
        HttpMethodsGetPeriodicRespone(HttpMethodsGet),
        HttpMethodsGetOnEventResponse(HttpMethodsGet),
        HttpMethodsGetOnEventRequest(()),
    }

    impl MsgDataBound for Data {}

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
        header: "HTTP response".into(),
    };

    let http_config = http_client_config::Config::<Data> {
        connection_config: http_client_config::ConnectionConfig {
            base_url: Url::parse("http://127.0.0.1:80")?,
        },
        requests_input: vec![http_client_config::RequestInput {
            fn_input: |msg| {
                let msg = msg.get_data()?;
                match msg {
                    Data::HttpMethodsGetOnEventRequest(_) => {
                        let param = http_client_config::HttpParam::Get("get".to_string());
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
        requests_periodic: vec![http_client_config::RequestPeriodic {
            period: Duration::from_secs(5),
            http_param: http_client_config::HttpParam::Get("get".to_string()),
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
        executor_name: "http_client".into(),
        fn_auth: |msg, _| Some(msg),
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
