use std::collections::HashMap;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::{main, time::Duration};
use tracing::{level_filters::LevelFilter, Level};
use url::Url;

use rsiot_http_client::cmp_http_client::{self, config};
use rsiot_messages_core::IMessage;

//------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    HttpMethodsGetPeriodicRespone(HttpMethodsGet),
    HttpMethodsGetOnEventResponse(HttpMethodsGet),
    HttpMethodsGetOnEventRequest,
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        todo!()
    }
}

//------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HttpMethodsGet {
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
    origin: String,
    url: String,
}

//------------------------------------------------------------------------------

#[main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let http_config = config::Config::<Message> {
        connection_config: config::ConnectionConfig {
            base_url: Url::parse("http://127.0.0.1:80").unwrap(),
        },
        requests_input: vec![config::RequestInput {
            fn_input: |msg| match msg {
                Message::HttpMethodsGetOnEventRequest => {
                    let param = config::HttpParam::Get("get".to_string());
                    Some(param)
                }
                _ => None,
            },
            on_success: |body| {
                let res = from_str::<HttpMethodsGet>(body).unwrap();
                vec![Message::HttpMethodsGetOnEventResponse(res)]
            },
            on_failure: || vec![],
        }],
        requests_periodic: vec![config::RequestPeriodic {
            period: Duration::from_secs(5),
            http_param: config::HttpParam::Get("get".to_string()),
            on_success: |body| {
                let res = from_str::<HttpMethodsGet>(&body).unwrap();
                vec![Message::HttpMethodsGetPeriodicRespone(res)]
            },
            on_failure: || vec![],
        }],
    };

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Message::HttpMethodsGetOnEventRequest;
                vec![msg]
            },
        }))
        .add_cmp(cmp_http_client::new(http_config))
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
            header: "HTTP response".into(),
        }));

    chain.spawn().await;
}
