use tokio::task::JoinSet;
use tracing::{info, warn};

use crate::{
    components_config::websocket_general::WebsocketMessage,
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
    serde_utils::SerdeAlg,
};

use super::{Config, Error, cmp_websocket_client_general::WebsocketClientGeneralTasks, tasks};

pub async fn fn_process<TMsg, TServerToClient, TClientToServer>(
    config: Config<TMsg, TServerToClient, TClientToServer>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TServerToClient: WebsocketMessage + 'static,
    TClientToServer: WebsocketMessage + 'static,
{
    info!("Starting cmp_websocket_client_wasm. Config: {config:?}");

    let serde_alg = SerdeAlg::new(config.serde_alg);

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Запуск общих задач
    let ws_general = WebsocketClientGeneralTasks {
        msgbus_linker,
        task_set: &mut task_set,
        fn_client_to_server: config.fn_client_to_server,
        fn_server_to_client: config.fn_server_to_client,
        serde_alg,
        fn_connection_state: config.fn_connection_state,
    };
    let (ch_rx_input_to_send, ch_tx_receive_to_output, ch_tx_connection_state) = ws_general.spawn();

    // Задачи коммуникации Websocket
    let task = tasks::SendReceive {
        url: config.url.clone(),
        ch_rx_input_to_send,
        ch_tx_receive_to_output,
        ch_tx_connection_state,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_websocket_client_wasm | send_receive",
        task.spawn(),
    );

    while let Some(task_result) = task_set.join_next().await {
        warn!("Task completed with result: {:?}", task_result);
        task_result??
    }

    Err(Error::FnProcessEnd)
}
