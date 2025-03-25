use std::time::Duration;

use futures::StreamExt;
use gloo::{net::websocket::futures::WebSocket, timers::future::sleep};
use tokio::{sync::mpsc, task::JoinSet};
use tracing::{info, warn};
use url::Url;

use crate::{
    components::cmp_websocket_client_general::{ConnectionState, WebsocketClientGeneralTasks},
    components_config::websocket_general::WebsocketMessage,
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{tasks, Config, Error};

pub async fn fn_process<TMessage, TServerToClient, TClientToServer>(
    config: Config<TMessage, TServerToClient, TClientToServer>,
    input: CmpInOut<TMessage>,
) -> super::Result<()>
where
    TMessage: MsgDataBound + 'static,
    TServerToClient: WebsocketMessage + 'static,
    TClientToServer: WebsocketMessage + 'static,
{
    info!("Starting cmp_websocket_client_wasm. Config: {config:?}");

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();
    let (ch_tx_connection_state, ch_rx_connection_state) = mpsc::channel(1000);
    let task = ConnectionState {
        input: ch_rx_connection_state,
        output: input.clone(),
        fn_connection_state: config.fn_connection_state,
    };
    join_set_spawn(&mut task_set, task.spawn());

    loop {
        let result = task_main(
            config.clone(),
            input.clone(),
            ch_tx_connection_state.clone(),
        )
        .await;
        warn!("End with resilt: {:?}", result);
        info!("Restarting...");
        ch_tx_connection_state.send(false).await.unwrap();
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage, TServerToClient, TClientToServer>(
    config: Config<TMessage, TServerToClient, TClientToServer>,
    msg_bus: CmpInOut<TMessage>,
    ch_tx_connection_state: mpsc::Sender<bool>,
) -> super::Result<()>
where
    TMessage: MsgDataBound + 'static,
    TServerToClient: WebsocketMessage + 'static,
    TClientToServer: WebsocketMessage + 'static,
{
    let url = Url::parse(&config.url).map_err(Error::BadUrl)?;
    let url = url.to_string();
    let ws = WebSocket::open(&url).map_err(|e| Error::SetupConnection(e.to_string()))?;
    info!("Connection to websocket server established");
    let (websocket_write, websocket_read) = ws.split();

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Запуск общих задач
    let ws_general = WebsocketClientGeneralTasks {
        msg_bus: msg_bus.clone(),
        buffer_size: 1000,
        task_set: &mut task_set,
        fn_client_to_server: config.fn_client_to_server,
        fn_server_to_client: config.fn_server_to_client,
        ch_tx_connection_state,
    };
    let (ch_rx_input_to_send, ch_tx_receive_to_output) = ws_general.spawn();

    // Задача отправки текста на сервер
    let task = tasks::Send {
        input: ch_rx_input_to_send,
        websocket_write,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Задача получения текста из сервера
    let task = tasks::Receive {
        websocket_read,
        output: ch_tx_receive_to_output,
    };
    join_set_spawn(&mut task_set, task.spawn());

    while let Some(task_result) = task_set.join_next().await {
        warn!("Task completed with result: {:?}", task_result);
        task_result??
    }
    Ok(())
}

// /// Задача отправки входящего потока сообщений на Websocker сервер
// async fn task_input<TMsg, TService>(
//     config: Config<TMsg>,
//     mut input: CmpInOut<TMsg, TService>,
//     mut write_stream: SplitSink<WebSocket, Message>,
// ) -> super::Result<()>
// where
//     TMsg: MsgDataBound,
//     TService: ServiceBound,
// {
//     while let Ok(msg) = input.recv_input().await {
//         let ws_msg = (config.fn_input)(&msg).map_err(Error::FnInput)?;
//         let ws_msg = match ws_msg {
//             Some(val) => val,
//             None => continue,
//         };
//         let ws_msg = Message::Text(ws_msg);
//         trace!("New message send to Websocker server: {:?}", ws_msg);
//         write_stream.send(ws_msg).await?;
//     }
//     Err(Error::TaskInput)
// }

// /// Задача получения текста из Websoket сервера и преобразование в исходящий поток сообщений
// async fn task_output<TMessage, TService>(
//     config: Config<TMessage>,
//     output: CmpInOut<TMessage, TService>,
//     mut read_stream: SplitStream<WebSocket>,
// ) -> super::Result<()>
// where
//     TMessage: MsgDataBound,
//     TService: ServiceBound,
// {
//     let mut first_execution = true;
//     while let Some(text) = read_stream.next().await {
//         trace!("New message from Websocket server: {:?}", text);
//         let text = match text {
//             Ok(text) => text,
//             Err(_) => continue,
//         };
//         let msg = match text {
//             Message::Text(value) => value,
//             Message::Bytes(_) => todo!(),
//         };

//         let msgs = (config.fn_output)(&msg).map_err(Error::FnOutput);
//         let msgs = match msgs {
//             Ok(val) => val,
//             Err(err) => {
//                 warn!("{err}");
//                 continue;
//             }
//         };

//         // Соединение установлено
//         if first_execution {
//             if let Some(msg) = (config.fn_connection_state)(true) {
//                 output.send_output(msg).await.map_err(Error::CmpOutput)?;
//             }
//             first_execution = false;
//         }

//         let Some(msgs) = msgs else { continue };
//         for msg in msgs {
//             output.send_output(msg).await.map_err(Error::CmpOutput)?;
//         }
//     }
//     // Соединение закрыто
//     if let Some(msg) = (config.fn_connection_state)(false) {
//         output.send_output(msg).await.map_err(Error::CmpOutput)?;
//     }
//     Err(Error::TaskOutput)
// }
