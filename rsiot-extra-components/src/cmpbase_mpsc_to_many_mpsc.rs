use rsiot_component_core::{StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;
use tracing::error;

/// Компонент для разделение одного потока в несколько
pub async fn new<TMessage>(input: StreamInput<TMessage>, outputs: Vec<StreamOutput<TMessage>>)
where
    TMessage: IMessage + 'static,
{
    let mut input = match input {
        Some(val) => val,
        None => {
            error!("Input stream not set, exit");
            return;
        }
    };

    while let Some(msg) = input.recv().await {
        for output in &outputs {
            let output = match output {
                Some(val) => val,
                None => continue,
            };
            let res = output.send(msg.clone()).await;
            if let Err(err) = res {
                error!("{}", err);
                return;
            }
        }
    }
}
