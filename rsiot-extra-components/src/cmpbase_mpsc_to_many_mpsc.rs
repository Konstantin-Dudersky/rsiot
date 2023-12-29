use rsiot_component_core::{ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;
use tracing::error;

/// Компонент для разделение одного потока в несколько
pub async fn new<TMessage>(
    mut input: ComponentInput<TMessage>,
    outputs: Vec<ComponentOutput<TMessage>>,
) where
    TMessage: IMessage + 'static,
{
    while let Ok(msg) = input.recv().await {
        for output in &outputs {
            let res = output.send(msg.clone()).await;
            if let Err(err) = res {
                error!("{}", err);
                return;
            }
        }
    }
}
