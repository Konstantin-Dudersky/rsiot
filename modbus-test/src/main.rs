use tokio::main;
use tokio_modbus::{client::Context, prelude::*};
use url::Url;

use messages::{IMessage, Messages};
use modbus_client_config::{
    client_config::{ClientConfig, TcpClientConfig},
    read::{ReadRequest, RequestParams, ResponseType},
};

#[main]
async fn main() {
    let url = Url::parse("tcp://127.0.0.1:502").unwrap();
    let sa = url.socket_addrs(|| None).unwrap();

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

    let socket_addr = "127.0.0.1:502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await.unwrap();

    // let data = request(&mut ctx, &modbus_read_config[0]).await;

    // println!("{:?}", data);
}

async fn read_request(
    ctx: &mut Context,
    req: &ReadRequest,
) -> Vec<Box<dyn IMessage>> {
    match req.params {
        RequestParams::ReadHoldingRegisters(address, count) => {
            let data =
                ctx.read_holding_registers(address, count).await.unwrap();
            let data = ResponseType::U16(data);
            (req.callback)(&data)
        }
        RequestParams::ReadCoils(_, _) => todo!(),
    }
}
