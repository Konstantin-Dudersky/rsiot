use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::{sleep, Duration},
};
use tokio_modbus::{client::Context, prelude::*};

use messages_lib::IMessage;
use modbus_client_config::{client_config::ClientConfig, read};

pub async fn client(
    // channel_write_to_modbus: Receiver<impl IMessage>,
    channel_read_from_modbus: Sender<Box<dyn IMessage>>,
    client_config: ClientConfig,
) {
    let (mut ctx, read_config) = match client_config {
        ClientConfig::Tcp(config) => {
            let socket_addr = format!(
                "{}:{}",
                config.url.host().unwrap(),
                config.url.port().unwrap()
            )
            .parse()
            .unwrap();

            (tcp::connect(socket_addr).await.unwrap(), config.read_config)
        }
        ClientConfig::Rtu => todo!(),
    };

    loop {
        for req in &read_config {
            let data = read_request(&mut ctx, req).await;
            for d in data {
                channel_read_from_modbus.send(d).await.unwrap();
            }
        }
        sleep(Duration::from_secs(2)).await;
    }
}

async fn read_request(
    ctx: &mut Context,
    req: &read::ReadRequest,
) -> Vec<Box<dyn IMessage>> {
    match req.params {
        read::RequestParams::ReadHoldingRegisters(address, count) => {
            let data =
                ctx.read_holding_registers(address, count).await.unwrap();
            let data = read::ResponseType::U16(data);
            (req.callback)(&data)
        }
        read::RequestParams::ReadCoils(_, _) => todo!(),
    }
}
