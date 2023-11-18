use tokio::{spawn, sync::mpsc};
use tracing::{debug, error, info, trace, warn, Level};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
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

/// Компонент для логгирования сообщений
pub async fn component_logger2<TMessage>(
    mut stream_input: Option<StreamInput<TMessage>>,
    stream_output: Option<StreamOutput<TMessage>>,
    config: u32,
) where
    TMessage: IMessage,
{
    let level = Level::INFO;
    let mut stream_input = stream_input.take().unwrap();
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

use rsiot_component_core::Component;

pub fn create_logger<TMessage>(config: u32) -> Component<TMessage, u32>
where
    TMessage: IMessage + 'static,
{
    Component::new(config, component_logger2)
}
