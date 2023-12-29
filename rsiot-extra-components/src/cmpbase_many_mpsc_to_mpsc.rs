use tokio::{spawn, sync::mpsc};

use rsiot_messages_core::IMessage;

/// Компонент для объединения нескольких потоков в один
pub async fn new<TMessage>(inputs: Vec<mpsc::Receiver<TMessage>>, output: mpsc::Sender<TMessage>)
where
    TMessage: IMessage + 'static,
{
    let (tx, mut rx) = mpsc::channel::<TMessage>(100);

    for mut stream in inputs {
        let tx_clone = tx.clone();
        spawn(async move {
            while let Some(msg) = stream.recv().await {
                tx_clone.send(msg).await.unwrap();
            }
        });
    }

    spawn(async move {
        while let Some(msg) = rx.recv().await {
            output.send(msg).await.unwrap();
        }
    })
    .await
    .unwrap();
}
