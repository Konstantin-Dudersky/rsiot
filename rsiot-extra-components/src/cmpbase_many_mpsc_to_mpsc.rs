use tokio::{sync::mpsc, task::JoinSet};

use rsiot_component_core::ComponentError;
use rsiot_messages_core::IMessage;

/// Компонент для объединения нескольких потоков в один
pub async fn new<TMessage>(
    inputs: Vec<mpsc::Receiver<TMessage>>,
    output: mpsc::Sender<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage + 'static,
{
    let (tx, mut rx) = mpsc::channel::<TMessage>(100);
    let mut task_set: JoinSet<Result<(), ComponentError>> = JoinSet::new();

    for mut stream in inputs {
        let tx_clone = tx.clone();
        task_set.spawn(async move {
            while let Some(msg) = stream.recv().await {
                tx_clone
                    .send(msg)
                    .await
                    .map_err(|err| ComponentError::Execution(err.to_string()))?;
            }
            Ok(())
        });
    }

    task_set.spawn(async move {
        while let Some(msg) = rx.recv().await {
            output
                .send(msg)
                .await
                .map_err(|err| ComponentError::Execution(err.to_string()))?;
        }
        Ok(())
    });

    while let Some(result) = task_set.join_next().await {
        result.map_err(|err| ComponentError::Execution(err.to_string()))??
    }

    Ok(())
}
