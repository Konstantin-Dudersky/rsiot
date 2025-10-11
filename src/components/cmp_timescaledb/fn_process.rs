use std::sync::Arc;

use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
};
use tracing::info;
use url::Url;

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{DatabasePool, Error, config::Config, tasks};

pub async fn fn_process<TMsg>(
    msgbus_linker: MsgBusLinker<TMsg>,
    config: Config<TMsg>,
) -> Result<(), Error>
where
    TMsg: 'static + MsgDataBound,
{
    info!("Start cmp_timescaledb");

    let pool: DatabasePool = Arc::new(Mutex::new(None));

    let connection_string = Url::parse(&config.connection_string)?;

    let (ch_tx_input_to_database, ch_rx_input_to_database) = mpsc::channel(1000);
    let (ch_tx_database_to_results, ch_rx_database_to_results) = mpsc::channel(10);

    let mut task_set = JoinSet::new();

    let task = tasks::SetupDatabase {
        pool: pool.clone(),
        connection_string,
        max_connections: config.max_connections,
        delete_before_write: config.delete_before_write,
        table_name: config.table_name,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_timescaledb | setup_database",
        task.spawn(),
    );

    let task = tasks::Input {
        msgbus_input: msgbus_linker.input(),
        output: ch_tx_input_to_database.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_timescaledb | input", task.spawn());

    let task = tasks::Periodic {
        output: ch_tx_input_to_database,
        period: config.send_period,
    };
    join_set_spawn(&mut task_set, "cmp_timescaledb | periodic", task.spawn());

    let task = tasks::SendToDatabase {
        input: ch_rx_input_to_database,
        output: ch_tx_database_to_results,
        table_name: config.table_name,
        max_cache_size: config.max_cache_size,
        pool: pool.clone(),
    };
    join_set_spawn(
        &mut task_set,
        "cmp_timescaledb | send_to_database",
        task.spawn(),
    );

    let task = tasks::CollectResults {
        input: ch_rx_database_to_results,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_timescaledb | collect_results",
        task.spawn(),
    );

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    // TODO - перезапуск при ошибке сохранения данных в БД

    Ok(())
}
