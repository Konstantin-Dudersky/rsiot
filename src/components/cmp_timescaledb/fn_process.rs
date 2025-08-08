use tokio::{
    sync::mpsc,
    task::JoinSet,
    time::{sleep, Duration},
};
use tracing::{error, info};
use url::Url;

use crate::{
    executor::{join_set_spawn, CmpInOut, ComponentError},
    message::MsgDataBound,
};

use super::{config::Config, tasks, Result};

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

    let (ch_tx_input_to_database, ch_rx_input_to_database) = mpsc::channel(1000);
    let (ch_tx_database_to_results, ch_rx_database_to_results) = mpsc::channel(10);

    let mut task_set = JoinSet::new();

    let task = tasks::Input {
        input: in_out.clone(),
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
        connection_string,
        max_connections: config.max_connections,
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

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
