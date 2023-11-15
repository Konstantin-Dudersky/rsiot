use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::{main, spawn};
use url::Url;

use rsiot_http_client::component_http_client;
use rsiot_http_client_config::{
    ConnectionConfig, HttpClientConfig, Request, RequestCyclic, RequestKind,
    RequestOnEvent,
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
    let config = HttpClientConfig::<Message> {
        connection_config: ConnectionConfig {
            url: Url::parse("http://127.0.0.1:80").unwrap(),
        },
        requests_on_event: vec![RequestOnEvent {}],
        requests_cyclic: vec![RequestCyclic {
            cycle: Duration::from_secs(5),
            request_params: Request {
                endpoint: "get".to_string(),
                kind: RequestKind::Get,
            },
            on_success: || Vec::<Message>::new(),
        }],
    };

    let task_http = spawn(component_http_client(config));

    task_http.await.unwrap();
}
