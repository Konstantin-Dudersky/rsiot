//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc influxdb_v2
//! ```

use std::time::Duration;

use crate::message::*;

use super::LineProtocolItem;

/// Функция преобразования входящих сообщений
pub type FnInput<TMsg> = fn(&Message<TMsg>) -> Option<Vec<LineProtocolItem>>;

// ANCHOR: Config
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
// ANCHOR: Config
