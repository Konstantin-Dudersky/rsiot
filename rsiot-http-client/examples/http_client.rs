//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-http-client --example http_client
//! ```

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use serde_json::from_str;
    use tokio::time::Duration;
    use tracing::{level_filters::LevelFilter, Level};
    use url::Url;

    use rsiot_component_core::ComponentExecutor;
    use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
    use rsiot_http_client::cmp_http_client::{self, config};
    use rsiot_messages_core::{msg_meta, IMessage, IMsgContentValue, MsgContent, MsgMeta};

    //------------------------------------------------------------------------------

    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
    enum Message {
        HttpMethodsGetPeriodicRespone(MsgContent<HttpMethodsGet>),
        HttpMethodsGetOnEventResponse(MsgContent<HttpMethodsGet>),
        HttpMethodsGetOnEventRequest(MsgContent<()>),
    }

    impl IMessage for Message {
        fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
            vec![]
        }
    }

    //------------------------------------------------------------------------------

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    struct HttpMethodsGet {
        args: HashMap<String, String>,
        headers: HashMap<String, String>,
        origin: String,
        url: String,
    }

    impl IMsgContentValue for HttpMethodsGet {
        fn fmt_value(&self, _template: &str) -> String {
            todo!()
        }
    }

    //------------------------------------------------------------------------------

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::HttpMethodsGetOnEventRequest(MsgContent::new(()));
            vec![msg]
        },
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "HTTP response".into(),
    };

    let http_config = config::Config::<Message> {
        connection_config: config::ConnectionConfig {
            base_url: Url::parse("http://127.0.0.1:80")?,
        },
        requests_input: vec![config::RequestInput {
            fn_input: |msg| match msg {
                Message::HttpMethodsGetOnEventRequest(_) => {
                    let param = config::HttpParam::Get("get".to_string());
                    Some(param)
                }
                _ => None,
            },
            on_success: |body| {
                let res = from_str::<HttpMethodsGet>(body)?;
                Ok(vec![Message::HttpMethodsGetOnEventResponse(
                    MsgContent::new(res),
                )])
            },
            on_failure: Vec::new,
        }],
        requests_periodic: vec![config::RequestPeriodic {
            period: Duration::from_secs(5),
            http_param: config::HttpParam::Get("get".to_string()),
            on_success: |body| {
                let res = from_str::<HttpMethodsGet>(body)?;
                Ok(vec![Message::HttpMethodsGetPeriodicRespone(
                    MsgContent::new(res),
                )])
            },
            on_failure: Vec::new,
        }],
    };

    ComponentExecutor::new(100)
        .add_cmp(cmp_http_client::Cmp::new(http_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
