use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::{error, info, trace};
use url::Url;

use rsiot_component_core::{StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

use crate::{config::Config, error::Error, row::Row};

pub async fn process<TMessage>(
    input: StreamInput<TMessage>,
    _output: StreamOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage,
{
    info!("Start timescaledb-storing");
    let mut input = match input {
        Some(val) => val,
        None => {
            let err = "Input stream not set, exit";
            error!(err);
            return;
        }
    };

    loop {
        let result =
            task_main::<TMessage>(&mut input, config.fn_process, &config.connection_string).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn task_main<TMessage>(
    input: &mut mpsc::Receiver<TMessage>,
    fn_process: fn(TMessage) -> Option<Row>,
    connection_string: &Url,
) -> Result<(), Error>
where
    TMessage: IMessage,
{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string.as_str())
        .await?;
    while let Some(msg) = input.recv().await {
        let row = fn_process(msg);
        if let Some(row) = row {
            save_row_in_db(&row, &pool).await?;
        };
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
