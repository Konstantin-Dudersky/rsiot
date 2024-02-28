//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc influxdb_v2
//! ```

use rsiot_messages_core::*;

use super::LineProtocolItem;

#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    /// # Примеры
    ///
    /// ```ignore
    /// host: String::from("influxdb"),
    /// ```
    pub host: String,
    pub port: u16,
    pub org: String,
    pub bucket: String,
    pub token: String,

    /// Функция преобразования сообщения в строки протокола InfluxDB
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// # use rsiot_components_config::influxdb_v2 as cmp_influxdb;
    /// # // insert from tests::stub
    /// # use rsiot_messages_core::example_message::*;
    /// # cmp_influxdb::Config::<Custom> {
    /// #     host: String::from("influxdb"),
    /// #     port: 8086,
    /// #     org: String::from("org"),
    /// #     bucket: String::from("bucket"),
    /// #     token: String::from("token"),
    /// fn_input: |_| None,
    /// # };
    /// ```
    ///
    /// ## Сохранение Custom
    ///
    /// ```rust
    /// # use rsiot_components_config::influxdb_v2 as cmp_influxdb;
    /// # // start tests::fn_input
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # cmp_influxdb::Config::<Custom> {
    /// #     host: String::from("influxdb"),
    /// #     port: 8086,
    /// #     org: String::from("org"),
    /// #     bucket: String::from("bucket"),
    /// #     token: String::from("token"),
    /// fn_input: |msg: &Message<Custom>| {
    ///     let value = match &msg.data {
    ///         MsgData::Custom(data) => match data {
    ///             Custom::ValueInstantF64(data) => {
    ///                 cmp_influxdb::ValueType::f64(*data)
    ///             }
    ///             _ => return None,
    ///         },
    ///         _ => return None,
    ///     };
    ///     let line = cmp_influxdb::LineProtocolItem::new(&msg.key, value, &msg.ts);
    ///     Some(vec![line])
    /// },
    /// # };
    /// # // end
    /// ```
    pub fn_input: fn(&Message<TMsg>) -> Option<Vec<LineProtocolItem>>,
}

#[cfg(test)]
mod test {
    use super::super::super::influxdb_v2 as cmp_influxdb;

    #[test]
    fn stub() {
        use rsiot_messages_core::example_message::*;
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
        use rsiot_messages_core::{example_message::*, *};
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
