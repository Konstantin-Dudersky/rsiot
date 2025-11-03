use std::time::Duration;

use rand::prelude::*;
use rsiot::components::cmp_tsdb_writer::*;
use tracing::info;

use super::message::*;

type FnInputReturn = Result<Option<Vec<String>>, Error>;

pub fn cmp(
    rows_in_cycle: u32,
    instance_number: String,
) -> Cmp<Msg, impl Fn(&Msg) -> FnInputReturn> {
    let hst = format!("hst_test_{}", instance_number);

    let row_builder = RowBuilder::prj("prj_test")
        .hst(hst)
        .svc("svc_test")
        .cmp("cmp_inject_periodic");

    let config = Config {
        _msg_phantom: std::marker::PhantomData,
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data".into(),
        max_connections: 10,
        table_name: "raw",
        save_by_row_count: 20_000,
        save_by_period: Duration::from_secs(2),
        fn_input: move |msg| {
            let mut rng = rand::rng();

            let rows = match msg {
                Msg::Counter(_) => (0..rows_in_cycle)
                    .map(|cn| {
                        let key = format!("counter_{:02}", cn);
                        let value: f64 = rng.random();
                        row_builder.row_without_ts(key, value).unwrap()
                    })
                    .collect::<Vec<String>>(),
            };
            Ok(Some(rows))
        },
        delete_before_write: false,
        fn_query_stat: |qs| info!("{}", qs.to_string()),
    };

    Cmp::new(config)
}
