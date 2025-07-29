use futures::TryFutureExt;
use tokio::{
    sync::mpsc,
    task::JoinSet,
    time::{sleep, Duration},
};
use tracing::{error, info};
use url::Url;

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, CmpInOut, ComponentError},
    message::MsgDataBound,
};

use super::{config::Config, error::Error, tasks, Result};

pub async fn fn_process<TMsg>(
    mut input: CmpInOut<TMsg>,
    config: Config<TMsg>,
) -> std::result::Result<(), ComponentError>
where
    TMsg: 'static + MsgDataBound,
{
    info!("Start cmp_timescaledb");

    loop {
        let result = task_main(&mut input, &config).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn task_main<TMsg>(in_out: &mut CmpInOut<TMsg>, config: &Config<TMsg>) -> Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let connection_string = Url::parse(&config.connection_string)?;

    let (ch_tx_msgbus_to_input, ch_rx_msgbus_to_input) = mpsc::channel(1000);
    let (ch_tx_input_to_database, ch_rx_input_to_database) = mpsc::channel(1000);

    let mut task_set = JoinSet::new();

    let task = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
        msg_bus: in_out.clone(),
        output: ch_tx_msgbus_to_input,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_timescaledb",
        task.spawn().map_err(Error::TaskMsgBusToMpsc),
    );

    let task = tasks::Input {
        input: ch_rx_msgbus_to_input,
        output: ch_tx_input_to_database.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_timescaledb", task.spawn());

    let task = tasks::Periodic {
        output: ch_tx_input_to_database,
        period: config.send_period,
    };
    join_set_spawn(&mut task_set, "cmp_timescaledb", task.spawn());

    let task = tasks::SendToDatabase {
        input: ch_rx_input_to_database,
        connection_string,
        max_connections: config.max_connections,
    };
    join_set_spawn(&mut task_set, "cmp_timescaledb", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}

// async fn save_row_in_db(row: Row, pool: Pool<Postgres>) -> Result<()> {
//     debug!("Save row in database: {:?}", row);
//     query(
//         r#"
// INSERT INTO raw
// VALUES ($1, $2, $3, $4, $5, $6, $7)
// ON CONFLICT (time, entity, attr, agg) DO UPDATE
//     SET value = excluded.value,
//          aggts = excluded.aggts,
//          aggnext = excluded.aggnext;"#,
//     )
//     .bind(row.time)
//     .bind(&row.entity)
//     .bind(&row.attr)
//     .bind(row.value)
//     .bind(&row.agg)
//     .bind(row.aggts)
//     .bind(&row.aggnext)
//     .execute(&pool)
//     .await?;
//     Ok(())
// }
