use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};
use tokio::time::{sleep, Duration};
use tracing::{error, info, trace};
use url::Url;

use rsiot_messages_core::MsgDataBound;

use crate::executor::{CmpInOut, ComponentError};

use super::{config::Config, error::Error, model::Row};

pub async fn fn_process<TMessage>(
    mut input: CmpInOut<TMessage>,
    config: Config,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound,
{
    info!("Start timescaledb-storing");

    loop {
        let result = task_main::<TMessage>(&mut input, &config.connection_string).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn task_main<TMessage>(
    input: &mut CmpInOut<TMessage>,
    connection_string: &Url,
) -> Result<(), Error>
where
    TMessage: MsgDataBound,
{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string.as_str())
        .await?;
    while let Ok(msg) = input.recv_input().await {
        // TODO
        // let msgs_eav = msg.into_eav();
        // for msg in msgs_eav {
        //     let row: Row = msg.into();
        //     save_row_in_db(&row, &pool).await?;
        // }
    }
    Ok(())
}

async fn save_row_in_db(row: &Row, pool: &Pool<Postgres>) -> Result<(), Error> {
    trace!("Save row in database: {:?}", row);
    query(
        r#"
INSERT INTO raw 
VALUES ($1, $2, $3, $4, $5::agg_type, $6, $7)
ON CONFLICT (ts, entity, attr, agg) DO UPDATE
    SET value = excluded.value,
        aggts = excluded.aggts,
        aggnext = excluded.aggnext;"#,
    )
    .bind(row.ts)
    .bind(&row.entity)
    .bind(&row.attr)
    .bind(row.value)
    .bind(&row.agg)
    .bind(row.aggts)
    .bind(&row.aggnext)
    .execute(pool)
    .await?;
    Ok(())
}
