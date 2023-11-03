mod errors;
mod types;

pub use errors::Errors;
pub use types::Result_;

use std::net::SocketAddr;

use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::{sleep, Duration},
};
use tokio_modbus::{client::Context, prelude::*};

use rsiot_messages_core::IMessage;
use rsiot_modbus_client_config::{client_config::ClientConfig, read, write};

pub async fn start_modbus_client<TMsg>(
    mut channel_write_to_modbus: Receiver<TMsg>,
    channel_read_from_modbus: Sender<TMsg>,
    client_config: ClientConfig<TMsg>,
) where
    TMsg: IMessage,
{
    let (mut ctx, read_config, write_config) = match client_config {
        ClientConfig::Tcp(config) => {
            let socket_addr = SocketAddr::new(config.host, config.port);
            (
                tcp::connect(socket_addr).await.unwrap(),
                config.read_config,
                config.write_config,
            )
        }
        ClientConfig::Rtu => todo!(),
    };

    loop {
        // проверяем, нет ли в канале сообщений для записи
        let msg = channel_write_to_modbus.try_recv();
        match msg {
            Ok(msg) => write_request(&mut ctx, &write_config, &msg).await,
            Err(_) => (),
        };

        // выполняем чтение регистров
        for req in &read_config {
            let data = read_request(&mut ctx, req).await.unwrap();
            for d in data {
                channel_read_from_modbus.send(d).await.unwrap();
            }
        }
        sleep(Duration::from_secs(2)).await;
    }
}

/// Выполняем запрос на чтение
async fn read_request<TMsg>(
    ctx: &mut Context,
    req: &read::Request<TMsg>,
) -> Result_<Vec<TMsg>>
where
    TMsg: IMessage,
{
    match req.params {
        read::RequestParams::ReadHoldingRegisters(address, count) => {
            let response = ctx.read_holding_registers(address, count).await;
            let response = match response {
                Ok(val) => val,
                Err(err) => {
                    let msg = format!(
                        "Error when read. Request: {:?}. Error: {:?}",
                        req, err
                    );
                    return Err(Errors::Read(msg));
                }
            };
            let response = read::ResponseType::U16(response);
            let response = (req.callback)(&response);
            Ok(response)
        }
        read::RequestParams::ReadCoils(_, _) => todo!(),
    }
}

/// Выполняем запрос на запись
async fn write_request<T>(
    ctx: &mut Context,
    req: &write::Request<T>,
    msg: &T,
) -> () {
    let param = (req.params)(msg);
    match param {
        write::RequestParams::NoRequest => (),
        write::RequestParams::WriteSingleRegister(start_address, value) => {
            ctx.write_single_register(start_address, value)
                .await
                .unwrap();
        }
    }
}
