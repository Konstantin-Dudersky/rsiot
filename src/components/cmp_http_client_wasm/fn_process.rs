use tokio::task::JoinSet;
use tracing::info;

use crate::{
    components::shared_tasks::cmp_http_client::HttpClientGeneral,
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{Result, config, tasks};

pub async fn fn_process<TMsg>(
    msgbus_linker: MsgBusLinker<TMsg>,
    config: config::Config<TMsg>,
) -> Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting http-client, configuration: {:?}", config);

    let mut task_set = JoinSet::new();

    let http_client_general = HttpClientGeneral {
        msgbus_linker,
        task_set: &mut task_set,
        requests_input: config.requests_input,
        requests_periodic: config.requests_periodic,
    };

    let (ch_rx_requests, ch_tx_reponse) = http_client_general.spawn();

    let task = tasks::HttpClient {
        input: ch_rx_requests,
        output: ch_tx_reponse,
        base_url: config.base_url,
    };
    join_set_spawn(&mut task_set, "cmp_http_client_wasm", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??;
    }
    Ok(())
}
