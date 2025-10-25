use std::{
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

use tokio::{sync::mpsc, task::JoinSet};
use tracing::info;

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{Error, Row, config::Config, tasks};

pub async fn fn_process<TMsg, TFnInput>(
    msgbus_linker: MsgBusLinker<TMsg>,
    config: Config<TMsg, TFnInput>,
) -> Result<(), Error>
where
    TMsg: 'static + MsgDataBound,
    TFnInput: 'static + Fn(&TMsg) -> Result<Option<Vec<Row>>, Error> + Send + Sync,
{
    info!("Start cmp_timescaledb");

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
        connection_string: config.connection_string,
        database_setup,
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

    Ok(())
}
