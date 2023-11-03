use std::net::{IpAddr, Ipv4Addr};

use tokio::{
    main, spawn,
    sync::mpsc::channel,
    time::{sleep, Duration},
};

use rsiot_messages_core::IMessage;
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

impl IMessage for Messages {}

#[main]
async fn main() {
    // конфигурация modbus клиента
    let modbus_client_config = ClientConfig::Tcp(TcpClientConfig {
        host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 502,
        read_config: vec![Request {
            params: RequestParams::ReadHoldingRegisters(0, 1),
            callback: |data| {
                let data = match data {
                    ResponseType::U16(data) => data,
                    ResponseType::Bool(_) => todo!(),
                };
                let msg = Messages::Reg0(data[0] as f64);
                vec![msg]
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

    // каналы для передачи сообщений
    let (modbus_write_tx, modbus_write_rx) = channel::<Messages>(128);
    let (modbus_read_tx, mut modbus_read_rx) = channel::<Messages>(128);

    // генерация значений для записи по modbus
    let mut counter = 0;
    let _write_task = spawn(async move {
        loop {
            modbus_write_tx
                .send(Messages::Reg0(counter as f64))
                .await
                .unwrap();
            counter += 1;
            sleep(Duration::from_secs(4)).await;
        }
    });

    // запуск modbus клиента
    let task = spawn(start_modbus_client(
        modbus_write_rx,
        modbus_read_tx,
        modbus_client_config,
    ));

    // обработка прочитанных данных из modbus
    let _read_task = spawn(async move {
        while let Some(r) = modbus_read_rx.recv().await {
            println!("{r:?}");
        }
    });

    task.await.unwrap();
}
