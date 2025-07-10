use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};
use tokio::time::{sleep, Duration};
use tokio_util::task::TaskTracker;
use tracing::{debug, error, info};
use url::Url;

use crate::{
    executor::{CmpInOut, ComponentError},
    message::MsgDataBound,
};

use super::{config::Config, error::Error, model::Row};

pub async fn fn_process<TMsg>(
    mut input: CmpInOut<TMsg>,
    config: Config<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
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

async fn task_main<TMsg>(input: &mut CmpInOut<TMsg>, config: &Config<TMsg>) -> Result<(), Error>
where
    TMsg: MsgDataBound,
{
    let connection_string = Url::parse(&config.connection_string)?;

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(connection_string.as_str())
        .await?;

    let task_tracker = TaskTracker::new();

    while let Ok(msg) = input.recv_input().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };

        let rows = (config.fn_input)(&msg);
        let Some(rows) = rows else { continue };

        for row in rows {
            let task = save_row_in_db(row, pool.clone());
            task_tracker.spawn(task);
        }
    }

    Ok(())
}

async fn save_row_in_db(row: Row, pool: Pool<Postgres>) -> Result<(), Error> {
    debug!("Save row in database: {:?}", row);
    query(
        r#"
INSERT INTO raw
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT (time, entity, attr, agg) DO UPDATE
    SET value = excluded.value,
         aggts = excluded.aggts,
         aggnext = excluded.aggnext;"#,
    )
    .bind(row.time)
    .bind(&row.entity)
    .bind(&row.attr)
    .bind(row.value)
    .bind(&row.agg)
    .bind(row.aggts)
    .bind(&row.aggnext)
    .execute(&pool)
    .await?;
    Ok(())
}
