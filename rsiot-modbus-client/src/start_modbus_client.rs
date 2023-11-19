use std::net::SocketAddr;

use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::{sleep, Duration},
};
use tokio_modbus::{client::Context, prelude::*};
use tracing::{error, info};

use rsiot_messages_core::IMessage;
use rsiot_modbus_client_config::{client_config::ClientConfig, read, write};

use crate::{errors::Errors, types::Result_};

pub async fn start_modbus_client<TMsg>(
    mut channel_write_to_modbus: Receiver<TMsg>,
    channel_read_from_modbus: Sender<TMsg>,
    client_config: ClientConfig<TMsg>,
) where
    TMsg: IMessage,
{
    loop {
        info!("Starting modbus client, configuration: {:?}", client_config);
        let res = start_modbus_client_loop::<TMsg>(
            &mut channel_write_to_modbus,
            &channel_read_from_modbus,
            client_config.clone(),
        )
        .await;
        match res {
            Ok(_) => (),
            Err(err) => {
                error!("Error in Modbus client: {:?}", err);
                sleep(Duration::from_secs(2)).await;
                info!("Restarting...");
            }
        }
    }
}

async fn start_modbus_client_loop<TMsg>(
    channel_write_to_modbus: &mut Receiver<TMsg>,
    channel_read_from_modbus: &Sender<TMsg>,
    client_config: ClientConfig<TMsg>,
) -> Result<(), Errors>
where
    TMsg: IMessage,
{
    let (mut ctx, read_config, write_config) = match client_config {
        ClientConfig::Tcp(config) => {
            let socket_addr = SocketAddr::new(config.host, config.port);
            (
                tcp::connect(socket_addr).await?,
                config.read_config,
                config.write_config,
            )
        }
        ClientConfig::Rtu => todo!(),
    };

    loop {
        // проверяем, нет ли в канале сообщений для записи
        let msg = channel_write_to_modbus.try_recv();
        if let Ok(msg) = msg {
            write_request(&mut ctx, &write_config, &msg).await?;
        }
        // выполняем чтение регистров
        for req in &read_config {
            let response = read_single_request(&mut ctx, req).await?;
            for msg in response {
                send_msg_to_channel(msg, channel_read_from_modbus).await?;
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}

/// Отправка прочитанных данных в канал передачи сообщений
async fn send_msg_to_channel<TMsg>(
    msg: TMsg,
    channel_read_from_modbus: &Sender<TMsg>,
) -> Result_<()> {
    let res = channel_read_from_modbus.send(msg).await;
    if let Err(err) = res {
        let err = err.to_string();
        return Err(Errors::ChannelSendError(err));
    }
    Ok(())
}

/// Выполняем один запрос на чтение
async fn read_single_request<TMsg>(
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
                    let err = format!(
                        "Error when read. Request: {:?}. Error: {:?}",
                        req.params, err
                    );
                    return Err(Errors::Read(err));
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
async fn write_request<TMsg>(
    ctx: &mut Context,
    req: &write::Request<TMsg>,
    msg: &TMsg,
) -> Result_<()>
where
    TMsg: IMessage,
{
    let param = (req.params)(msg);
    match param {
        write::RequestParams::NoRequest => Ok(()),
        write::RequestParams::WriteSingleRegister(start_address, value) => {
            let response =
                ctx.write_single_register(start_address, value).await;
            match response {
                Ok(_) => (),
                Err(err) => {
                    let err = format!(
                        "Error when write. Request: {:?}. Error: {}",
                        param, err
                    );
                    return Err(Errors::Write(err));
                }
            }
            Ok(())
        }
    }
}
