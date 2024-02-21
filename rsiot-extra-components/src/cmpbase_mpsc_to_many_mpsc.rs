use tracing::error;

use rsiot_component_core::{CmpInput, CmpOutput};
use rsiot_messages_core::message_v2::MsgDataBound;

/// Компонент для разделение одного потока в несколько
pub async fn new<TMessage>(mut input: CmpInput<TMessage>, outputs: Vec<CmpOutput<TMessage>>)
where
    TMessage: MsgDataBound + 'static,
{
    while let Ok(msg) = input.recv().await {
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        for output in &outputs {
            let res = output.send(msg.clone()).await;
            if let Err(err) = res {
                error!("{}", err);
                return;
            }
        }
    }
}
