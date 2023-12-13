//! Пример для работы с ферментером UST.

use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::cmp_logger;
use rsiot_messages_core::{msg_types, IMessage};
use rsiot_modbus_client::cmp_modbus_client::{self, *};

use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Messages {
    Temperature(msg_types::Value<f64>),
}

impl IMessage for Messages {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}

const TO_F32: fn(&[u16]) -> f32 = conversion::to_f32::little_endian_swap;

#[main]
async fn main() {
    // логгирование
    fmt().with_max_level(LevelFilter::DEBUG).init();

    // конфигурация modbus клиента
    let modbus_client_config = cmp_modbus_client::Config::Tcp(TcpClientConfig::<Messages> {
        host: IpAddr::V4(Ipv4Addr::new(10, 0, 6, 10)),
        port: 502,
        unit_id: 1,
        input_config: vec![InputConfig {
            fn_input: |_| None,
            fn_on_success: |_| vec![],
            fn_on_failure: Vec::new,
        }],
        periodic_config: vec![PeriodicConfig {
            period: Duration::from_secs(2),
            request: Request::ReadHoldingRegisters(0, 2),
            fn_on_success: |data| {
                let data = match data {
                    Response::U16(data) => data,
                    _ => return vec![],
                };
                let temperature = TO_F32(&data[0..=1]) as f64;
                vec![Messages::Temperature(msg_types::Value::new(temperature))]
            },
            fn_on_failure: || vec![Messages::Temperature(msg_types::Value::new(0.0))],
        }],
    });

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_modbus_client::new(modbus_client_config))
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));

    chain.spawn().await;
}
