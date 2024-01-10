use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{
    net::TcpStream,
    task::JoinSet,
    time::{sleep, Duration},
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{error, info, warn};

use rsiot_component_core::{ComponentError, ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

use crate::{config::Config, error::Error};

pub async fn fn_process<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
) -> Result<(), ComponentError>
where
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
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage + 'static,
{
    let (ws_stream, _) = connect_async(config.url).await?;
    let (write, read) = ws_stream.split();

    let mut task_set: JoinSet<Result<(), Error<TMessage>>> = JoinSet::new();
    task_set.spawn(task_send(input, write, config.fn_input));
    task_set.spawn(task_recv(output, read, config.fn_output));

    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

/// Задача отправки данных на сервер Websocket
async fn task_send<TMessage>(
    mut input: ComponentInput<TMessage>,
    mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    fn_send: fn(&TMessage) -> Option<String>,
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        let text = (fn_send)(&msg);
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
    fn_recv: fn(&str) -> anyhow::Result<Vec<TMessage>>,
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage,
{
    while let Some(msg) = read.next().await {
        let data = msg?.into_text()?;
        let msgs = (fn_recv)(&data).map_err(|err| Error::FnOutput(err))?;
        for msg in msgs {
            output.send(msg).await?;
        }
    }
    Ok(())
}
