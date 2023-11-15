use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::{main, spawn};
use tracing::{error, level_filters::LevelFilter};
use url::Url;

use rsiot_http_client::component_http_client;
use rsiot_http_client_config::{
    ConnectionConfig, HttpClientConfig, RequestCyclic, RequestOnEvent,
    RequestParam, RequestParamKind,
};
use rsiot_messages_core::IMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

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
        requests_cyclic: vec![RequestCyclic {
            cycle: Duration::from_secs(5),
            request_params: RequestParam {
                endpoint: "get1".to_string(),
                kind: RequestParamKind::Get,
            },
            on_success: |body| {
                println!("{:?}", body);
                Vec::<Message>::new()
            },
            on_failure: || {
                error!("error");
                vec![]
            },
        }],
    };

    let task_http = spawn(component_http_client(config));

    task_http.await.unwrap();
}
