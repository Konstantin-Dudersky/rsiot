use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::IMessage;
use rsiot_modbus_client::cmp_modbus_client::{self, *};

use tracing::Level;
use tracing_subscriber::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Messages {
    Value0(f64),
}

impl IMessage for Messages {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        todo!()
    }
}

#[main]
async fn main() {
    // логгирование
    fmt().init();

    // конфигурация modbus клиента
    let modbus_client_config = Config::<Messages>::Tcp(TcpClientConfig {
        host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 5020,
        input_config: vec![InputConfig {
            fn_input: |msg| match msg {
                Messages::Value0(val) => Some(Request::WriteSingleRegister(0, *val as u16)),
            },
            fn_on_success: |_data| vec![],
            fn_on_failure: || vec![],
        }],
        periodic_config: vec![PeriodicConfig {
            period: Duration::from_secs(2),
            request: Request::ReadHoldingRegisters(0, 1),
            fn_on_success: |data| {
                let mut msgs = vec![];
                if let Response::U16(data) = data {
                    msgs.push(Messages::Value0(data[0] as f64));
                }
                msgs
            },
            fn_on_failure: || vec![],
        }],
    });

    let mut counter = 0.0;
    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Messages::Value0(counter);
                counter += 1.0;
                vec![msg]
            },
        }))
        .add_cmp(cmp_modbus_client::new(modbus_client_config))
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));

    chain.spawn().await;
}
