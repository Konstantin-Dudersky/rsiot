use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{
    net::TcpStream,
    spawn,
    task::JoinHandle,
    time::{sleep, Duration},
    try_join,
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{error, info, warn};

use rsiot_component_core::{ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

use crate::{config::Config, Error};

pub async fn fn_process<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    info!("cmp_websocket_client starting");

    loop {
        let res = task_connect(input.resubscribe(), output.clone(), config.clone()).await;
        match res {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        warn!("Restaring...");
        sleep(Duration::from_secs(2)).await;
    }
}

/// Подключаемся к серверу и запускаем потоки получения и отправки
async fn task_connect<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
) -> Result<(), Error>
where
    TMessage: IMessage + 'static,
{
    let (ws_stream, _) = connect_async(config.url).await?;
    let (write, read) = ws_stream.split();
    let task_send = spawn(task_send(input, write, config.fn_send));
    let task_recv = spawn(task_recv(output, read, config.fn_recv));
    try_join!(flatten(task_recv), flatten(task_send)).map(|_| ())
}

/// Задача отправки данных на сервер Websocket
async fn task_send<TMessage>(
    mut input: ComponentInput<TMessage>,
    mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    fn_send: fn(TMessage) -> Option<String>,
) -> Result<(), Error>
where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        let text = (fn_send)(msg);
        if let Some(text) = text {
            let text = Message::Text(text);
            write.send(text).await?;
        }
    }
    Ok(())
}

/// Задача приема данных с сервера Websocket
async fn task_recv<TMessage>(
    output: ComponentOutput<TMessage>,
    mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    fn_recv: fn(String) -> Vec<TMessage>,
) -> Result<(), Error>
where
    TMessage: IMessage,
{
    while let Some(msg) = read.next().await {
        let data = msg?.into_text()?;
        let msgs = (fn_recv)(data);
        for msg in msgs {
            output.send(msg).await?;
        }
    }
    Ok(())
}

async fn flatten<T>(handle: JoinHandle<Result<T, Error>>) -> Result<T, Error> {
    handle.await?
}
