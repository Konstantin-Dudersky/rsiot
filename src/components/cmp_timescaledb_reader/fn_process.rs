use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use tokio::{sync::Semaphore, task::JoinSet, time::sleep};
use tracing::info;
use url::Url;

use crate::{
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, Error, tasks};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let connection_string = Url::parse(&config.connection_string)?;

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(connection_string.as_str())
        .await
        .map_err(Error::PgConnectionError)?;

    let concurrent_connections = Arc::new(Semaphore::new(config.max_connections as usize));

    let mut task_set = JoinSet::new();

    for item in config.items {
        let task = tasks::Read {
            msgbus_output: msgbus_linker.output(),
            database_pool: pool.clone(),
            concurrent_connections: concurrent_connections.clone(),
            time_begin: config.time_begin,
            time_end: config.time_end,
            entity: item.entity,
            attr: item.attr,
            fn_output: item.fn_output,
            delay_between_msgs: config.delay_between_msgs,
        };

        join_set_spawn(&mut task_set, "cmp_tsdb_reader", task.spawn());
    }

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    sleep(config.shutdown_delay).await;
    info!("Reader tasks have been shut down");

    Ok(())
}
