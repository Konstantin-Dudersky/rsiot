use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{
    net::TcpStream,
    task::JoinSet,
    time::{sleep, Duration},
};
use tokio_tungstenite::{
    connect_async, tungstenite::Message as TungsteniteMessage, MaybeTlsStream, WebSocketStream,
};
use tracing::{error, info, warn};
use url::Url;

use crate::{
    executor::{CmpInOut, ComponentError},
    message::{Message, MsgDataBound, ServiceBound},
};

use super::{
    config::{Config, FnOutput},
    error::Error,
};

pub async fn fn_process<TMessage, TService>(
    input: CmpInOut<TMessage, TService>,
    config: Config<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    info!("cmp_websocket_client starting");

    loop {
        let res = task_connect(input.clone(), config.clone()).await;
        match res {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        warn!("Restaring...");
        sleep(Duration::from_secs(2)).await;
    }
}

/// Подключаемся к серверу и запускаем потоки получения и отправки
async fn task_connect<TMessage, TService>(
    in_out: CmpInOut<TMessage, TService>,
    config: Config<TMessage>,
) -> Result<(), Error<TMessage>>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    let url = Url::parse(&config.url).map_err(Error::BadUrl)?;

    let (ws_stream, _) = connect_async(url).await?;
    let (write, read) = ws_stream.split();

    let mut task_set: JoinSet<Result<(), Error<TMessage>>> = JoinSet::new();
    task_set.spawn(task_send(in_out.clone(), write, config.fn_input));
    task_set.spawn(task_recv(in_out, read, config.fn_output));

    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

/// Задача отправки данных на сервер Websocket
async fn task_send<TMessage, TService>(
    mut input: CmpInOut<TMessage, TService>,
    mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteMessage>,
    fn_send: fn(&Message<TMessage>) -> anyhow::Result<Option<String>>,
) -> Result<(), Error<TMessage>>
where
    TMessage: MsgDataBound,
    TService: ServiceBound,
{
    while let Ok(msg) = input.recv_input().await {
        let text = (fn_send)(&msg).map_err(Error::FnInput)?;
        if let Some(text) = text {
            let text = TungsteniteMessage::Text(text);
            write.send(text).await?;
        }
    }
    Ok(())
}

/// Задача приема данных с сервера Websocket
async fn task_recv<TMessage, TService>(
    output: CmpInOut<TMessage, TService>,
    mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    fn_recv: FnOutput<TMessage>,
) -> Result<(), Error<TMessage>>
where
    TMessage: MsgDataBound,
    TService: ServiceBound,
{
    while let Some(msg) = read.next().await {
        let data = msg?.into_text()?;
        let msgs = (fn_recv)(&data).map_err(|err| Error::FnOutput(err))?;
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
