use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::{sleep, Duration},
};
use tokio_modbus::{client::Context, prelude::*};

use rsiot_modbus_client_config::{client_config::ClientConfig, read, write};

pub async fn start_modbus_client<TMsg>(
    mut channel_write_to_modbus: Receiver<TMsg>,
    channel_read_from_modbus: Sender<TMsg>,
    client_config: ClientConfig<TMsg>,
) {
    let (mut ctx, read_config, write_config) = match client_config {
        ClientConfig::Tcp(config) => {
            let socket_addr = format!(
                "{}:{}",
                config.url.host().unwrap(),
                config.url.port().unwrap()
            )
            .parse()
            .unwrap();

            (
                tcp::connect(socket_addr).await.unwrap(),
                config.read_config,
                config.write_config,
            )
        }
        ClientConfig::Rtu => todo!(),
    };

    loop {
        let msg = channel_write_to_modbus.try_recv();
        match msg {
            Ok(msg) => write_request(&mut ctx, &write_config, &msg).await,
            Err(_) => (),
        };

        for req in &read_config {
            let data = read_request(&mut ctx, req).await;
            for d in data {
                channel_read_from_modbus.send(d).await.unwrap();
            }
        }
        sleep(Duration::from_secs(2)).await;
    }
}

async fn read_request<T>(ctx: &mut Context, req: &read::Request<T>) -> Vec<T> {
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
