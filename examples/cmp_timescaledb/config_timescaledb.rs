use std::time::Duration;

use rsiot::{components::cmp_timescaledb::*, executor::Component};

use super::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data".into(),
        max_connections: 5,
        table_name: "raw",
        send_period: Duration::from_secs(2),
        fn_input: |msg| {
            let row = match msg {
                Msg::Counter(v) => Row::new_simple("counter", "value", *v as f64),
            };
            Some(vec![row])
        },
        delete_before_write: false,
    };

    Cmp::new(config)
}
