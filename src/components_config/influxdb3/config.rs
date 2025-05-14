//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc influxdb_v2
//! ```

use std::time::Duration;

use crate::message::*;

use super::LineProtocolItem;

pub type FnInput<TMsg> = fn(&Message<TMsg>) -> Option<Vec<LineProtocolItem>>;

/// Конфигурация cmp_influxdb
#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    /// # Примеры
    ///
    /// ```ignore
    /// host: String::from("influxdb"),
    /// ```
    pub host: String,

    /// Порт базы данных
    pub port: u16,

    /// База данных
    pub database: String,

    /// Периодичность отправки данных для сохранения в базе данных
    pub send_period: Duration,

    /// Функция преобразования сообщения в строки протокола InfluxDB
    pub fn_input: FnInput<TMsg>,
}

#[cfg(test)]
mod test {
    use super::super::super::influxdb_v2 as cmp_influxdb;

    #[test]
    fn stub() {
        use crate::message::example_message::*;
        let _ = cmp_influxdb::Config::<Custom> {
            host: String::from("influxdb"),
            port: 8086,
            org: String::from("org"),
            bucket: String::from("bucket"),
            token: String::from("token"),
            fn_input: |_| None,
        };
    }

    #[test]
    fn fn_input() {
        use crate::message::{example_message::*, *};
        let _ = cmp_influxdb::Config::<Custom> {
            host: String::from("influxdb"),
            port: 8086,
            org: String::from("org"),
            bucket: String::from("bucket"),
            token: String::from("token"),
            fn_input: |msg: &Message<Custom>| {
                let value = match &msg.data {
                    MsgData::Custom(Custom::ValueInstantF64(data)) => {
                        cmp_influxdb::ValueType::f64(*data)
                    }
                    _ => return None,
                };
                let line = cmp_influxdb::LineProtocolItem::new(&msg.key, value, &msg.ts);
                Some(vec![line])
            },
        };
    }
}
