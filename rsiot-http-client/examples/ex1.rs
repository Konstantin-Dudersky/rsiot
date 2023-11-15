use serde::{Deserialize, Serialize};
use tokio::{main, spawn};

use rsiot_http_client::component_http_client;
use rsiot_http_client_config::{ConnectionConfig, HttpClientConfig};
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
        connection_config: ConnectionConfig { url: () },
        requests_on_event: todo!(),
        requests_cyclic: todo!(),
    };

    let task_http = spawn(component_http_client(config));

    task_http.await.unwrap();
}
