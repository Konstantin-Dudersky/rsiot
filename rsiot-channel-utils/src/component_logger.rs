use tokio::sync::mpsc;
use tracing::{debug, error, info, trace, warn, Level};

use rsiot_messages_core::IMessage;

/// Компонент для логгирования сообщений
pub async fn component_logger<TMessage>(
    mut stream_input: mpsc::Receiver<TMessage>,
    stream_output: Option<mpsc::Sender<TMessage>>,
    level: Level,
) where
    TMessage: IMessage,
{
    while let Some(msg) = stream_input.recv().await {
        match level {
            Level::TRACE => trace!("{:?}", msg),
            Level::DEBUG => debug!("{:?}", msg),
            Level::INFO => info!("{:?}", msg),
            Level::WARN => warn!("{:?}", msg),
            Level::ERROR => error!("{:?}", msg),
        }
        match &stream_output {
            Some(stream) => stream.send(msg).await.unwrap(),
            None => (),
        }
    }
}
