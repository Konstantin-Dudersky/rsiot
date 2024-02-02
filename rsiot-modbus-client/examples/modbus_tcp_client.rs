//! Простейший пример использования клиента Modbus
//!
//! Для тестирования можно использовать образ docker oitc/modbus-server
//!
//! Выполняется две операции:
//! - раз в 2 секунды на сервер в регистр 0 записывается значение счетчика (`input_config`)
//! - раз в 2 секунды считывается значение регистра 0 (`periodic_config`) и отправляется в логгер
//!
//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-modbus-client --example modbus_tcp_client
//! ```

use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};
use tracing::Level;
use tracing_subscriber::fmt;

use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{msg_meta, IMessage, IMsgContentValue, MsgContent, MsgMeta};
use rsiot_modbus_client::cmp_modbus_client::{self, *};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
pub enum Messages {
    ValueWrite(MsgContent<f64>),
    ValueRead(MsgContent<f64>),
}

impl IMessage for Messages {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}

#[main]
async fn main() -> anyhow::Result<()> {
    // логгирование
    fmt().init();

    // Конфигурация modbus клиента
    let modbus_client_config = Config {
        enabled: true,
        unit_id: 1,
        input_config: vec![InputConfig {
            fn_input: |msg| match msg {
                Messages::ValueWrite(val) => {
                    Some(Request::WriteSingleRegister(0, val.value as u16))
                }
                Messages::ValueRead(_) => None,
            },
            fn_on_success: |_data| vec![],
            fn_on_failure: Vec::new,
        }],
        periodic_config: vec![PeriodicConfig {
            period: Duration::from_secs(2),
            request: Request::ReadHoldingRegisters(0, 1),
            fn_on_success: |data| {
                let mut msgs = vec![];
                if let Response::U16(data) = data {
                    msgs.push(Messages::ValueRead(MsgContent::new(data[0] as f64)));
                }
                msgs
            },
            fn_on_failure: Vec::new,
        }],
        client_type: ClientType::Tcp(TcpClientType {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5020,
        }),
    };

    let mut counter = 0.0;
    ComponentExecutor::new(100)
        // Периодическое генерирование сообщения для записи счетчика на сервер
        .add_cmp(cmp_inject_periodic::Cmp::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Messages::ValueWrite(MsgContent::new(counter));
                counter += 1.0;
                vec![msg]
            },
        }))
        // Клиент modbus
        .add_cmp(cmp_modbus_client::Cmp::new(modbus_client_config))
        // Вывод сообщений в лог
        .add_cmp(cmp_logger::Cmp::new(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }))
        .wait_result()
        .await?;
    Ok(())
}
