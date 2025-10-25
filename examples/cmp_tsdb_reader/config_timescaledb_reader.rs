use std::time::Duration;

use rsiot::{components::cmp_timescaledb_reader::*, executor::Component};
use time::macros::datetime;

use super::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data".into(),
        max_connections: 5,
        time_begin: datetime!(2025-08-19 07:20:00.000+03),
        time_end: datetime!(2025-08-19 07:20:10.000+03),
        items: vec![ConfigItem {
            entity: "accelerometer",
            attr: "accel_x",
            fn_output: |value| Msg::MTsdbReader(MTsdbReader::AccelX(value)),
        }],
        delay_between_msgs: Duration::from_millis(1),
        shutdown_delay: Duration::from_secs(5),
    };
    Cmp::new(config)
}
