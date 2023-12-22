use tokio::{spawn, sync::mpsc};

use rsiot_component_core::{StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;
use tracing::error;

/// Компонент для объединения нескольких потоков в один
pub async fn new<TMessage>(inputs: Vec<StreamInput<TMessage>>, output: StreamOutput<TMessage>)
where
    TMessage: IMessage + 'static,
{
    let output = match output {
        Some(val) => val,
        None => {
            error!("Output stream not set, exit");
            return;
        }
    };

    let (tx, mut rx) = mpsc::channel::<TMessage>(100);

    for stream in inputs {
        let mut stream = match stream {
            Some(val) => val,
            None => continue,
        };
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
