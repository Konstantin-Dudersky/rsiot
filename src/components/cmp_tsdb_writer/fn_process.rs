use std::{
    str::FromStr,
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

use sqlx::{
    ConnectOptions,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use tokio::{sync::mpsc, task::JoinSet};
use tracing::info;

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{COMPONENT_NAME, Error, config::Config, tasks};

pub async fn fn_process<TMsg, TFnInput>(
    msgbus_linker: MsgBusLinker<TMsg>,
    config: Config<TMsg, TFnInput>,
) -> Result<(), Error>
where
    TMsg: 'static + MsgDataBound,
    TFnInput: 'static + Fn(&TMsg) -> Result<Option<Vec<String>>, Error> + Send + Sync,
{
    info!("Start {COMPONENT_NAME}");

    let database_setup = Arc::new(AtomicBool::new(false));

    let (ch_tx_input_to_database, ch_rx_input_to_database) = mpsc::channel(1000);
    let (ch_tx_database_to_results, ch_rx_database_to_results) = mpsc::channel(10);

    let mut task_set = JoinSet::new();

    let task = tasks::SetupDatabase {
        connection_string: config.connection_string.clone(),
        delete_before_write: config.delete_before_write,
        table_name: config.table_name,
        database_setup: database_setup.clone(),
        reconnect_interval: Duration::from_millis(1_000),
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | setup_database"),
        task.spawn(),
    );

    let task = tasks::Input {
        msgbus_input: msgbus_linker.input(),
        output: ch_tx_input_to_database.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | input"),
        task.spawn(),
    );

    msgbus_linker.close();

    let task = tasks::Periodic {
        output: ch_tx_input_to_database,
        save_by_period: config.save_by_period,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | periodic"),
        task.spawn(),
    );

    let conn_options = PgConnectOptions::from_str(&config.connection_string)?
        .disable_statement_logging()
        // TODO - возможно вынести в конфигурацию
        .options([("statement_timeout", "1500")]) // в ms
        // TODO - возможно вынести в конфигурацию
        .application_name("cmp_tsdb_writer");
    let pool_options = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_slow_threshold(Duration::from_millis(1_000));
    let db_pool = pool_options.connect_with(conn_options).await?;

    let task = tasks::PrepareSQL {
        input: ch_rx_input_to_database,
        output: ch_tx_database_to_results,
        table_name: config.table_name,
        database_setup,
        db_pool,
        save_by_period: config.save_by_period,
        save_by_row_count: config.save_by_row_count,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | prepare_sql"),
        task.spawn(),
    );

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
