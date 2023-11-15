use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::{main, spawn, sync::mpsc};
use tracing::{info, level_filters::LevelFilter};
use url::Url;

use rsiot_http_client::component_http_client;
use rsiot_http_client_config::{
    ConnectionConfig, HttpClientConfig, RequestOnEvent, RequestParam,
    RequestPeriodic,
};
use rsiot_messages_core::IMessage;

//------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    HttpMethodsGet(HttpMethodsGet),
}

impl IMessage for Message {}

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

    let config = HttpClientConfig::<Message> {
        connection_config: ConnectionConfig {
            url: Url::parse("http://127.0.0.1:80").unwrap(),
        },
        requests_on_event: vec![RequestOnEvent {}],
        requests_periodic: vec![RequestPeriodic {
            period: Duration::from_secs(5),
            request_param: RequestParam::Get("get".to_string()),
            on_success: |body| {
                let res = from_str::<HttpMethodsGet>(&body).unwrap();
                vec![Message::HttpMethodsGet(res)]
            },
            on_failure: || vec![],
        }],
    };

    let (stream_client_out, mut stream_end) = mpsc::channel::<Message>(10);

    let task_http =
        spawn(component_http_client(None, Some(stream_client_out), config));

    let _task_end = spawn(async move {
        while let Some(msg) = stream_end.recv().await {
            info!("New output message: {:?}", msg);
        }
    });

    task_http.await.unwrap();
}
