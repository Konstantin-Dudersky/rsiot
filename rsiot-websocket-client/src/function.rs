use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{
    net::TcpStream,
    spawn,
    sync::{broadcast, mpsc},
    task::JoinHandle,
    time::{sleep, Duration},
    try_join,
};
use tokio_tungstenite::{
    connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};
use tracing::{error, info};

use rsiot_channel_utils::{cmp_mpsc_to_mpsc, cmpbase_mpsc_to_broadcast};
use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

use crate::{cmp_websocket_client, Error};

pub async fn function<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: cmp_websocket_client::Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    info!("cmp_websocket_client starting");

    let (from_server_tx, from_server_rx) = mpsc::channel::<TMessage>(100);
    let (input_broadcast_tx, _input_broadcast_rx) =
        broadcast::channel::<TMessage>(100);
    let input_broadcast_tx_clone = input_broadcast_tx.clone();

    let mut output_stream = cmp_mpsc_to_mpsc::create::<TMessage>();
    output_stream.set_input_output(Some(from_server_rx), output);
    output_stream.spawn();

    spawn(cmpbase_mpsc_to_broadcast::create(input, input_broadcast_tx));

    loop {
        let res = task_connect(
            input_broadcast_tx_clone.subscribe(),
            from_server_tx.clone(),
            config.clone(),
        )
        .await;
        match res {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restaring...");
        sleep(Duration::from_secs(2)).await;
    }
}

/// Подключаемся к серверу и запускаем потоки получения и отправки
async fn task_connect<TMessage>(
    input: broadcast::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    config: cmp_websocket_client::Config<TMessage>,
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
    mut input: broadcast::Receiver<TMessage>,
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
    stream_output: mpsc::Sender<TMessage>,
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
            stream_output.send(msg).await?;
        }
    }
    Ok(())
}

async fn flatten<T>(handle: JoinHandle<Result<T, Error>>) -> Result<T, Error> {
    handle.await?
}
