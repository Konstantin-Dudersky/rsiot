use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{Mutex, mpsc},
    task::{JoinHandle, JoinSet},
};
use tracing::info;

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{COMPONENT_NAME, Error, QueryStat, config::Config, config::ConfigTableForSetup, tasks};

pub async fn fn_process<TMsg>(
    msgbus_linker: MsgBusLinker<TMsg>,
    config: Config<TMsg>,
) -> Result<(), Error>
where
    TMsg: 'static + MsgDataBound,
{
    info!("Start {COMPONENT_NAME}");

    let (ch_tx_database_to_results, ch_rx_database_to_results) =
        mpsc::channel::<JoinHandle<Result<QueryStat, Error>>>(10);

    let mut task_set = JoinSet::new();

    let db_pool_mutex = Arc::new(Mutex::new(None));

    let tables_setup: Vec<ConfigTableForSetup> = config.tables.iter().map(|t| t.into()).collect();
    let task = tasks::SetupDatabase {
        connection_string: config.connection_string.connection_string(),
        reconnect_interval: Duration::from_millis(1_000),
        tables: tables_setup,
        db_pool_mutex: db_pool_mutex.clone(),
        max_connections: config.max_connections,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | setup_database"),
        task.spawn(),
    );

    for table in config.tables {
        let (tx_input_to_sql, rx_input_to_sql) = mpsc::channel::<tasks::InnerMessage>(1000);

        let table_name = table.table_name();

        let task = tasks::Input {
            msgbus_input: msgbus_linker.input(),
            output: tx_input_to_sql.clone(),
            table,
        };
        join_set_spawn(
            &mut task_set,
            format!("{COMPONENT_NAME} | input | {}", table_name),
            task.spawn(),
        );

        let task = tasks::Periodic {
            output: tx_input_to_sql,
            save_by_period: config.save_by_period,
        };
        join_set_spawn(
            &mut task_set,
            format!("{COMPONENT_NAME} | periodic | {}", table_name),
            task.spawn(),
        );

        let task = tasks::PrepareSQL {
            input: rx_input_to_sql,
            output: ch_tx_database_to_results.clone(),
            table_name: table_name.clone(),
            db_pool_mutex: db_pool_mutex.clone(),
            save_by_period: config.save_by_period,
            save_by_row_count: config.save_by_row_count,
        };
        join_set_spawn(
            &mut task_set,
            format!("{COMPONENT_NAME} | prepare_sql | {}", table_name),
            task.spawn(),
        );
    }

    msgbus_linker.close();

    let task = tasks::WaitResult {
        input: ch_rx_database_to_results,
        fn_query_stat: config.fn_query_stat,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | wait_result"),
        task.spawn(),
    );

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
