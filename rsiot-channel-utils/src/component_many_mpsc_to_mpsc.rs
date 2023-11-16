use tokio::{spawn, sync::mpsc};

use rsiot_messages_core::IMessage;

pub async fn component_many_mpsc_to_mpsc<TMessage>(
    streams_input: Vec<mpsc::Receiver<TMessage>>,
    stream_output: mpsc::Sender<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    let (tx, mut rx) = mpsc::channel::<TMessage>(100);

    for mut stream in streams_input {
        let tx_clone = tx.clone();
        spawn(async move {
            while let Some(msg) = stream.recv().await {
                tx_clone.send(msg).await.unwrap();
            }
        });
    }

    spawn(async move {
        while let Some(msg) = rx.recv().await {
            stream_output.send(msg).await.unwrap();
        }
    })
    .await
    .unwrap();
}
