use futures::TryFutureExt;
use tokio::{sync::mpsc, task::JoinSet};
use tracing::info;

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{config::Config, error::Error, tasks};

pub async fn fn_process<TMsg>(in_out: CmpInOut<TMsg>, config: Config<TMsg>) -> Result<(), Error>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting influxdb client, configuration: {:?}", config);

    let url = format!(
        "http://{host}:{port}/api/v3/write_lp",
        host = config.host,
        port = config.port,
    );

    let (ch_tx_msgbus_to_input, ch_rx_msgbus_to_input) = mpsc::channel(1000);
    let (ch_tx_input_to_database, ch_rx_input_to_database) = mpsc::channel(1000);

    let mut task_set = JoinSet::new();

    let task = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
        msg_bus: in_out,
        output: ch_tx_msgbus_to_input,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_influxdb3",
        task.spawn().map_err(Error::TaskMsgBusToMpsc),
    );

    let task = tasks::Input {
        input: ch_rx_msgbus_to_input,
        output: ch_tx_input_to_database.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_influxdb3", task.spawn());

    let task = tasks::Periodic {
        output: ch_tx_input_to_database,
        period: config.send_period,
    };
    join_set_spawn(&mut task_set, "cmp_influxdb3", task.spawn());

    let task = tasks::SendToDatabase {
        input: ch_rx_input_to_database,
        url,
        database: config.database,
    };
    join_set_spawn(&mut task_set, "cmp_influxdb3", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
