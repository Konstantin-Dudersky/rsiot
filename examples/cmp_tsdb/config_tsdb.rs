use std::time::Duration;

use rsiot::components::cmp_tsdb::*;

use super::message::*;

type FnInputReturn = Result<Option<Vec<Row>>, Error>;

pub fn cmp() -> Cmp<Msg, impl Fn(&Msg) -> FnInputReturn> {
    let config = Config {
        _msg_phantom: std::marker::PhantomData,
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data".into(),
        max_connections: 5,
        max_cache_size: 10_000,
        table_name: "raw",
        send_period: Duration::from_secs(2),
        fn_input: |msg| {
            let row = match msg {
                Msg::Counter(v) => RowBuilder::new()
                    .hst("hst_test")
                    .svc("svc_test")
                    .cmp("cmp_inject_periodic")
                    .key("counter")
                    .value(*v)
                    .row()?,
            };
            Ok(Some(vec![row]))
        },
        delete_before_write: false,
    };

    Cmp::new(config)
}
