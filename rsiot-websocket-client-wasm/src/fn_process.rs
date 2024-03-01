use std::time::Duration;

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo::{
    net::websocket::{futures::WebSocket, Message},
    timers::future::sleep,
};
use tokio::task::JoinSet;
use tracing::{info, trace, warn};

use rsiot_component_core::CmpInOut;
use rsiot_components_config::websocket_client::Config;
use rsiot_messages_core::MsgDataBound;

use crate::error::Error;

pub async fn fn_process<TMessage>(
    config: Config<TMessage>,
    input: CmpInOut<TMessage>,
) -> crate::Result
where
    TMessage: MsgDataBound + 'static,
{
    info!("Starting cmp_websocket_client_wasm. Config: {config:?}");
    loop {
        let result = task_main(config.clone(), input.clone()).await;
        warn!("End with resilt: {:?}", result);
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage>(config: Config<TMessage>, input: CmpInOut<TMessage>) -> crate::Result
where
    TMessage: MsgDataBound + 'static,
{
    let url = config.url.to_string();
    let ws = WebSocket::open(&url).map_err(Error::Connect)?;
    info!("Connection to websocket server established");
    let (write_stream, read_stream) = ws.split();

    let mut task_set: JoinSet<crate::Result> = JoinSet::new();
    task_set.spawn_local(task_input(config.clone(), input.clone(), write_stream));
    task_set.spawn_local(task_output(config, input, read_stream));

    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

/// Задача отправки входящего потока сообщений на Websocker сервер
async fn task_input<TMsg>(
    config: Config<TMsg>,
    mut input: CmpInOut<TMsg>,
    mut write_stream: SplitSink<WebSocket, Message>,
) -> crate::Result
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = input.recv_input().await {
        let msg = match msg {
            Some(msg) => msg,
            None => continue,
        };
        let ws_msg = (config.fn_input)(&msg).map_err(Error::FnInput)?;
        let ws_msg = match ws_msg {
            Some(val) => val,
            None => continue,
        };
        let ws_msg = Message::Text(ws_msg);
        trace!("New message send to Websocker server: {:?}", ws_msg);
        write_stream.send(ws_msg).await?;
    }
    Ok(())
}

/// Задача получения текста из Websoket сервера и преобразование в исходящий поток сообщений
async fn task_output<TMessage>(
    config: Config<TMessage>,
    output: CmpInOut<TMessage>,
    mut read_stream: SplitStream<WebSocket>,
) -> crate::Result
where
    TMessage: MsgDataBound,
{
    while let Some(text) = read_stream.next().await {
        trace!("New message from Websocket server: {:?}", text);
        let text = match text {
            Ok(text) => text,
            Err(_) => continue,
        };
        let msg = match text {
            Message::Text(value) => value,
            Message::Bytes(_) => todo!(),
        };
        let msgs = (config.fn_output)(&msg).map_err(Error::FnOutput)?;
        let msgs = match msgs {
            Some(msgs) => msgs,
            None => continue,
        };
        for msg in msgs {
            output.send_output(msg).await.map_err(Error::CmpOutput)?;
        }
    }
    Ok(())
}
