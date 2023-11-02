use tokio::{main, spawn, sync::mpsc::channel};
use url::Url;

use messages::Messages;
use modbus_client::client;
use modbus_client_config::{
    client_config::{ClientConfig, TcpClientConfig},
    read::{ReadRequest, RequestParams, ResponseType},
};

#[main]
async fn main() {
    let url = Url::parse("tcp://127.0.0.1:502").unwrap();

    let read_config = vec![ReadRequest {
        params: RequestParams::ReadHoldingRegisters(0, 1),
        callback: |data| {
            let data = match data {
                ResponseType::U16(data) => data,
                ResponseType::Bool(_) => todo!(),
            };
            let val = Messages::Reg0(data[0] as f64);
            vec![Box::new(val)]
        },
    }];

    let modbus_client_config = ClientConfig::Tcp(TcpClientConfig {
        url: url,
        read_config: read_config,
    });

    let (from_modbus_tx, mut from_modbus_rx) = channel(128);

    let task = spawn(client(from_modbus_tx, modbus_client_config));

    let _ = spawn(async move {
        while let Some(r) = from_modbus_rx.recv().await {
            println!("{r:?}");
        }
    });

    task.await.unwrap();
}
