use tokio::task::JoinSet;
use tracing::info;

use crate::{
    components::shared_tasks::cmp_http_client::HttpClientGeneral,
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{config, tasks, Result};

pub async fn fn_process<TMsg>(msg_bus: CmpInOut<TMsg>, config: config::Config<TMsg>) -> Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting http-client, configuration: {:?}", config);

    let mut task_set = JoinSet::new();

    let http_client_general = HttpClientGeneral {
        msg_bus,
        buffer_size: 1000,
        task_set: &mut task_set,
        requests_input: config.requests_input,
        requests_periodic: config.requests_periodic,
    };

    let (ch_rx_requests, ch_tx_reponse) = http_client_general.spawn();

    let task = tasks::HttpClient {
        input: ch_rx_requests,
        output: ch_tx_reponse,
        base_url: config.base_url,
        timeout: config.timeout,
    };
    join_set_spawn(&mut task_set, "cmp_http_client", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??;
    }
    Ok(())
}
