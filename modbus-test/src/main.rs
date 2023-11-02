use tokio::{main, spawn, sync::mpsc::channel};
use url::Url;

use messages::Messages;
use modbus_client::client;
use modbus_client_config::{
    client_config::{ClientConfig, TcpClientConfig},
    read::{ReadRequest, RequestParams, ResponseType},
    write,
};

#[main]
async fn main() {
    let read_config = vec![ReadRequest {
        params: RequestParams::ReadHoldingRegisters(0, 1),
        callback: |data| {
            let data = match data {
                ResponseType::U16(data) => data,
                ResponseType::Bool(_) => todo!(),
            };
            let val = Messages::Reg0(data[0] as f64);
            vec![val]
        },
    }];

    let write_config = vec![write::Request::<Messages> {
        params: |msg| match msg {
            Messages::Reg0(value) => {
                write::RequestParams::WriteSingleRegister(0, *value as u16)
            }
        },
    }];

    let modbus_client_config = ClientConfig::Tcp(TcpClientConfig {
        url: Url::parse("tcp://192.168.122.55:502").unwrap(),
        read_config,
        write_config,
    });

    let (modbus_write_tx, mut modbus_write_rx) = channel::<Messages>(128);
    let (modbus_read_tx, mut modbus_read_rx) = channel::<Messages>(128);

    let task = spawn(client(
        modbus_write_rx,
        modbus_read_tx,
        modbus_client_config,
    ));

    let _ = spawn(async move {
        while let Some(r) = modbus_read_rx.recv().await {
            println!("{r:?}");
        }
    });

    task.await.unwrap();
}
