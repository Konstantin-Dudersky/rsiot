use futures_util::{future, pin_mut, stream::SplitStream, StreamExt};
use tokio::{io::AsyncWriteExt, net::TcpStream, spawn, sync::mpsc};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use rsiot_messages_core::IMessage;

pub async fn function<TMessage>(
    stream_input: mpsc::Receiver<TMessage>,
    stream_output: mpsc::Sender<TMessage>,
    fn_from_server: fn(String) -> Vec<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    let (ws_stream, _) = connect_async("ws://localhost:9001")
        .await
        .expect("Failed to connect");

    let (write, read) = ws_stream.split();

    let task_from_server_to_client =
        spawn(from_server_to_client(stream_output, read, fn_from_server));

    task_from_server_to_client.await.unwrap()
}

async fn from_server_to_client<TMessage>(
    stream_output: mpsc::Sender<TMessage>,
    mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    fn_from_server: fn(String) -> Vec<TMessage>,
) {
    while let Some(msg) = read.next().await {
        let data = msg.unwrap().into_text().unwrap();
        let msgs = (fn_from_server)(data);
        for msg in msgs {
            stream_output.send(msg).await.unwrap();
        }
    }
}
