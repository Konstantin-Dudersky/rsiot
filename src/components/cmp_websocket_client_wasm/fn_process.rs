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
use url::Url;

use crate::{
    executor::{join_set_spawn, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

use super::{Config, Error};

pub async fn fn_process<TMessage, TService>(
    config: Config<TMessage>,
    input: CmpInOut<TMessage, TService>,
) -> super::Result
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    info!("Starting cmp_websocket_client_wasm. Config: {config:?}");
    loop {
        let result = task_main(config.clone(), input.clone()).await;
        warn!("End with resilt: {:?}", result);
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage, TService>(
    config: Config<TMessage>,
    msg_bus: CmpInOut<TMessage, TService>,
) -> super::Result
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    let url = Url::parse(&config.url).map_err(Error::BadUrl)?;
    let url = url.to_string();
    let ws = WebSocket::open(&url).map_err(Error::Connect)?;
    info!("Connection to websocket server established");
    let (write_stream, read_stream) = ws.split();

    let mut task_set: JoinSet<super::Result> = JoinSet::new();

    // Отправка входящих сообщений на Websocket сервер
    let task = task_input(config.clone(), msg_bus.clone(), write_stream);
    join_set_spawn(&mut task_set, task);

    // Данные от сервера в исходящий поток сообщений
    let task = task_output(config, msg_bus, read_stream);
    join_set_spawn(&mut task_set, task);

    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

/// Задача отправки входящего потока сообщений на Websocker сервер
async fn task_input<TMsg, TService>(
    config: Config<TMsg>,
    mut input: CmpInOut<TMsg, TService>,
    mut write_stream: SplitSink<WebSocket, Message>,
) -> super::Result
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    while let Ok(msg) = input.recv_input().await {
        let ws_msg = (config.fn_input)(&msg).map_err(Error::FnInput)?;
        let ws_msg = match ws_msg {
            Some(val) => val,
            None => continue,
        };
        let ws_msg = Message::Text(ws_msg);
        trace!("New message send to Websocker server: {:?}", ws_msg);
        write_stream.send(ws_msg).await?;
    }
    Err(Error::TaskInput)
}

/// Задача получения текста из Websoket сервера и преобразование в исходящий поток сообщений
async fn task_output<TMessage, TService>(
    config: Config<TMessage>,
    output: CmpInOut<TMessage, TService>,
    mut read_stream: SplitStream<WebSocket>,
) -> super::Result
where
    TMessage: MsgDataBound,
    TService: ServiceBound,
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

        let msgs = (config.fn_output)(&msg).map_err(Error::FnOutput);
        let msgs = match msgs {
            Ok(val) => val,
            Err(err) => {
                warn!("{err}");
                continue;
            }
        };

        let Some(msgs) = msgs else { continue };
        for msg in msgs {
            output.send_output(msg).await.map_err(Error::CmpOutput)?;
        }
    }
    Err(Error::TaskOutput)
}
