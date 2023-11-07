use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};
use tokio::{
    sync::mpsc::Receiver,
    time::{sleep, Duration},
};
use tracing::{error, info, trace};
use url::Url;

use rsiot_messages_core::IMessage;

use crate::{AggType, Error, Row};

pub async fn start_timescaledb_storing<TMessage>(
    mut channel_rcv: Receiver<TMessage>,
    config: fn(TMessage) -> Option<Row>,
    db_url: Url,
) -> ()
where
    TMessage: IMessage,
{
    loop {
        info!("Start timescaledb-storing");
        let result = start_timescaledb_storing_loop::<TMessage>(
            &mut channel_rcv,
            config,
            &db_url,
        )
        .await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn start_timescaledb_storing_loop<TMessage>(
    channel_rcv: &mut Receiver<TMessage>,
    config: fn(TMessage) -> Option<Row>,
    db_url: &Url,
) -> Result<(), Error>
where
    TMessage: IMessage,
{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;
    while let Some(msg) = channel_rcv.recv().await {
        let row = config(msg);
        if let Some(row) = row {
            save_row_in_db(&row, &pool).await.unwrap();
        };
    }
    Ok(())
}

pub async fn save_row_in_db(
    row: &Row,
    pool: &Pool<Postgres>,
) -> Result<(), Error> {
    trace!("Save row in database: {:?}", row);
    let _ = query!(
        r#" 
INSERT INTO raw
VALUES ($1, $2, $3, $4, $5::agg_type, $6)
ON CONFLICT (ts, entity, attr, agg) DO UPDATE
    SET value = excluded.value,
        aggts = excluded.aggts,
        aggnext = excluded.aggnext;"#,
        row.ts,
        row.entity,
        row.attr,
        row.value,
        row.agg.clone() as AggType,
        row.aggts,
    )
    .execute(pool)
    .await?;
    Ok(())
}
