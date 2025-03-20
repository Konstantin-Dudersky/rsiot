use futures::StreamExt;
use tokio::{
    sync::mpsc,
    task::JoinSet,
    time::{sleep, Duration},
};
use tokio_tungstenite::connect_async;
use tracing::{error, info, warn};
use url::Url;

use crate::{
    components::cmp_websocket_client_general::{ConnectionState, WebsocketClientGeneralTasks},
    components_config::websocket_general::WebsocketMessage,
    executor::{join_set_spawn, CmpInOut, ComponentError},
    message::{MsgDataBound, ServiceBound},
};

use super::{config::Config, tasks, Error};

pub async fn fn_process<TMessage, TService, TServerToClient, TClientToServer>(
    input: CmpInOut<TMessage, TService>,
    config: Config<TMessage, TServerToClient, TClientToServer>,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    info!("cmp_websocket_client starting");

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();
    let (ch_tx_connection_state, ch_rx_connection_state) = mpsc::channel(1000);
    let task = ConnectionState {
        input: ch_rx_connection_state,
        output: input.clone(),
        fn_connection_state: config.fn_connection_state,
    };
    join_set_spawn(&mut task_set, task.spawn());

    loop {
        let res = task_connect(
            input.clone(),
            config.clone(),
            ch_tx_connection_state.clone(),
        )
        .await;
        match res {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        warn!("Restaring...");
        ch_tx_connection_state.send(false).await.unwrap();
        sleep(Duration::from_millis(2000)).await;
    }
}

/// Подключаемся к серверу и запускаем потоки получения и отправки
async fn task_connect<TMessage, TService, TServerToClient, TClientToServer>(
    in_out: CmpInOut<TMessage, TService>,
    config: Config<TMessage, TServerToClient, TClientToServer>,
    ch_tx_connection_state: mpsc::Sender<bool>,
) -> super::Result<()>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    let url = Url::parse(config.url)?;

    let (ws_stream, _) = connect_async(url)
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
        task_set.shutdown().await;
    }
    Ok(())
}

// /// Задача отправки данных на сервер Websocket
// async fn task_send<TMessage, TService>(
//     mut input: CmpInOut<TMessage, TService>,
//     mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteMessage>,
//     fn_send: fn(&Message<TMessage>) -> anyhow::Result<Option<String>>,
// ) -> Result<(), Error<TMessage>>
// where
//     TMessage: MsgDataBound,
//     TService: ServiceBound,
// {
//     while let Ok(msg) = input.recv_input().await {
//         let text = (fn_send)(&msg).map_err(Error::FnInput)?;
//         if let Some(text) = text {
//             let text = TungsteniteMessage::Text(text);
//             write.send(text).await?;
//         }
//     }
//     Ok(())
// }

// /// Задача приема данных с сервера Websocket
// async fn task_recv<TMessage, TService>(
//     output: CmpInOut<TMessage, TService>,
//     mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
//     fn_recv: FnOutput<TMessage>,
// ) -> Result<(), Error<TMessage>>
// where
//     TMessage: MsgDataBound,
//     TService: ServiceBound,
// {
//     while let Some(msg) = read.next().await {
//         let data = msg?.into_text()?;
//         let msgs = (fn_recv)(&data).map_err(|err| Error::FnOutput(err))?;
//         let msgs = match msgs {
//             Some(msgs) => msgs,
//             None => continue,
//         };
//         for msg in msgs {
//             output.send_output(msg).await.map_err(Error::CmpOutput)?;
//         }
//     }
//     Ok(())
// }
