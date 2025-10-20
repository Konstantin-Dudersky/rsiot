use std::sync::{Arc, atomic::AtomicBool};

use sqlx::postgres::PgPoolOptions;
use tokio::task::JoinSet;
use url::Url;

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, Error, prepare_sql_statement::prepare_sql_statement, tasks};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
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

    let input_lagged = Arc::new(AtomicBool::new(false));

    let mut task_set = JoinSet::new();

    // Задача проверки загруженности входного канала сообщений
    let task = tasks::CheckLagged {
        input: msgbus_linker.input(),
        input_lagged: input_lagged.clone(),
    };
    join_set_spawn(
        &mut task_set,
        "cmp_tsdb_reader | check_lagged",
        task.spawn(),
    );

    let mut task = tasks::Read {
        msgbus_output: msgbus_linker.output(),
        database_pool: pool.clone(),
        delay_between_msgs: config.delay_between_msgs,
        input_lagged,
    };

    let output_shutdown = msgbus_linker.output();

    msgbus_linker.close();

    for item in config.items {
        let sql =
            prepare_sql_statement(config.time_begin, config.time_end, item.entity, item.attr)?;

        task.fetch(&sql, item.fn_output).await?;
    }

    let msg = (config.fn_shutdown)();
    let msg = msg.to_message();
    output_shutdown
        .send(msg)
        .await
        .map_err(|_| Error::TokioSyncMpscSend)?;

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
