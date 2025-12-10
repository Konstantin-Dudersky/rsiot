use std::time::Duration;

use rand::prelude::*;
use rsiot::components::cmp_tsdb_writer::*;
use tracing::info;

use super::message::*;

pub fn cmp(rows_in_cycle: u32, instance_number: String) -> Cmp<Msg> {
    let hst = format!("hst_test_{}", instance_number);

    // Таблица
    let fields = (0..10)
        .map(|cn| ConfigTableField {
            field_name: format!("counter_{:02}", cn),
            data_type: ConfigTableFieldType::NumericDoublePrecision,
        })
        .collect::<Vec<_>>();

    let table = ConfigTable {
        prj: "prj_test".into(),
        hst,
        svc: "svc_test".into(),
        cmp: "cmp_inject_periodic".into(),
        tag: "tag_test".into(),
        delete_before_write: true,
        chunk_interval: Duration::from_hours(1),
        compress_interval: Duration::from_hours(1),
        retention_interval: Some(Duration::from_hours(24)),
        fields,
        fn_input: |msg| {
            let mut rng = rand::rng();

            let rows = match msg {
                Msg::Counter(_) => {
                    let values = (0..10)
                        .map(|_| {
                            let value: f64 = rng.random();
                            format!("{}", value)
                        })
                        .collect::<Vec<String>>();
                    row_without_ts(&values).unwrap()
                }
            };
            Ok(Some(rows))
        },
    };

    let config = Config {
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data".into(),
        max_connections: 10,
        tables: vec![table],
        save_by_row_count: 20_000,
        save_by_period: Duration::from_secs(2),
        fn_query_stat: |qs| info!("{}", qs.to_string()),
    };

    Cmp::new(config)
}
