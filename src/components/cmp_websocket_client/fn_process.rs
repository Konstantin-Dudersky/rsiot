use futures::StreamExt;
use tokio::{
    sync::mpsc,
    task::JoinSet,
    time::{Duration, sleep},
};
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};
use tracing::{error, info, warn};

use crate::{
    components_config::websocket_general::WebsocketMessage,
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
    serde_utils::SerdeAlg,
};

use super::{
    Error,
    cmp_websocket_client_general::{ConnectionState, WebsocketClientGeneralTasks},
    config::Config,
    tasks,
};

pub async fn fn_process<TMessage, TServerToClient, TClientToServer>(
    input: CmpInOut<TMessage>,
    config: Config<TMessage, TServerToClient, TClientToServer>,
) -> Result<(), Error>
where
    TMessage: MsgDataBound + 'static,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    info!("cmp_websocket_client starting");

    let serde_alg = SerdeAlg::new(config.serde_alg);

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();
    let (ch_tx_connection_state, ch_rx_connection_state) = mpsc::channel(1000);
    let task = ConnectionState {
        input: ch_rx_connection_state,
        output: input.clone(),
        fn_connection_state: config.fn_connection_state,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_websocket_client | connection_state",
        task.spawn(),
    );

    loop {
        let res = task_connect(
            input.clone(),
            config.clone(),
            ch_tx_connection_state.clone(),
            serde_alg,
        )
        .await;
        match res {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        warn!("Restaring...");
        ch_tx_connection_state
            .send(false)
            .await
            .map_err(|_| Error::TokioSyncMpscSend)?;
        sleep(Duration::from_millis(2000)).await;
    }
}

/// Подключаемся к серверу и запускаем потоки получения и отправки
async fn task_connect<TMessage, TServerToClient, TClientToServer>(
    in_out: CmpInOut<TMessage>,
    config: Config<TMessage, TServerToClient, TClientToServer>,
    ch_tx_connection_state: mpsc::Sender<bool>,
    serde_alg: SerdeAlg,
) -> super::Result<()>
where
    TMessage: MsgDataBound + 'static,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    let request = config
        .url
        .into_client_request()
        .map_err(|e| Error::SetupConnection(e.to_string()))?;

    let (ws_stream, _) = connect_async(request)
        .await
        .map_err(|e| Error::SetupConnection(e.to_string()))?;

    let (websocket_write, websocket_read) = ws_stream.split();

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Запуск общих задач
    let ws_general = WebsocketClientGeneralTasks {
        msg_bus: in_out.clone(),
        buffer_size: 1000,
        task_set: &mut task_set,
        fn_client_to_server: config.fn_client_to_server,
        fn_server_to_client: config.fn_server_to_client,
        ch_tx_connection_state: ch_tx_connection_state.clone(),
        serde_alg,
    };
    let (ch_rx_input_to_send, ch_tx_receive_to_output) = ws_general.spawn();

    // Задача отправки текста на сервер
    let task = tasks::Send {
        input: ch_rx_input_to_send,
        websocket_write,
    };
    join_set_spawn(&mut task_set, "cmp_websocket_client", task.spawn());

    // Задача получения текста из сервера
    let task = tasks::Receive {
        websocket_read,
        output: ch_tx_receive_to_output,
    };
    join_set_spawn(&mut task_set, "cmp_websocket_client", task.spawn());

    while let Some(task_result) = task_set.join_next().await {
        warn!("Task completed with result: {:?}", task_result);
        task_set.shutdown().await;
    }
    Ok(())
}
