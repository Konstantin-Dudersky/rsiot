use tokio::main;
use tokio_modbus::{client::Context, prelude::*};

use messages::{IMessage, Messages};
use modbus_core::{ReadRequest, RequestParams, ResponseType};

#[main]
async fn main() {
    let cfg = vec![ReadRequest {
        request_params: RequestParams::ReadHoldingRegisters(0, 1),
        response_func: |data| {
            let data = match data {
                ResponseType::U16(data) => data,
                ResponseType::Bool(_) => todo!(),
            };
            let val = Messages::Reg0(data[0] as f64);
            vec![Box::new(val)]
        },
    }];

    let socket_addr = "192.168.101.34:502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await.unwrap();

    let data = request(&mut ctx, &cfg[0]).await;

    println!("{:?}", data);
}

async fn request<T>(
    ctx: &mut Context,
    req: &ReadRequest<T>,
) -> Vec<Box<dyn IMessage>>
where
    T: Fn(&ResponseType) -> Vec<Box<dyn IMessage>>,
{
    match req.request_params {
        RequestParams::ReadHoldingRegisters(address, count) => {
            let data =
                ctx.read_holding_registers(address, count).await.unwrap();
            let data = ResponseType::U16(data);
            (req.response_func)(&data)
        }
        RequestParams::ReadCoils(_, _) => todo!(),
    }
}
