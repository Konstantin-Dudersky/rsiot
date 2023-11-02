use tokio::{main, spawn, sync::mpsc::channel};
use url::Url;

use rsiot_modbus_client::start_modbus_client;
use rsiot_modbus_client_config::{
    client_config::{ClientConfig, TcpClientConfig},
    read::{Request, RequestParams, ResponseType},
    write,
};

#[derive(Debug)]
pub enum Messages {
    Reg0(f64),
}

#[main]
async fn main() {
    let modbus_client_config = ClientConfig::Tcp(TcpClientConfig {
        url: Url::parse("tcp://192.168.122.55:502").unwrap(),
        read_config: vec![Request {
            params: RequestParams::ReadHoldingRegisters(0, 1),
            callback: |data| {
                let data = match data {
                    ResponseType::U16(data) => data,
                    ResponseType::Bool(_) => todo!(),
                };
                let val = Messages::Reg0(data[0] as f64);
                vec![val]
            },
        }],
        write_config: write::Request {
            params: |msg| match msg {
                Messages::Reg0(value) => {
                    write::RequestParams::WriteSingleRegister(0, *value as u16)
                }
            },
        },
    });

    let (modbus_write_tx, modbus_write_rx) = channel::<Messages>(128);
    let (modbus_read_tx, mut modbus_read_rx) = channel::<Messages>(128);

    let task = spawn(start_modbus_client(
        modbus_write_rx,
        modbus_read_tx,
        modbus_client_config,
    ));

    let _ = spawn(async move {
        while let Some(r) = modbus_read_rx.recv().await {
            println!("{r:?}");
        }
    });
    modbus_write_tx.send(Messages::Reg0(121.0)).await.unwrap();

    task.await.unwrap();
}
