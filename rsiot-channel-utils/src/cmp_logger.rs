//! Компонент для логгирования сообщений

use tracing::{debug, error, info, trace, warn, Level};

use rsiot_component_core::{Component, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

async fn cmp_logger<TMessage>(
    mut stream_input: StreamInput<TMessage>,
    stream_output: StreamOutput<TMessage>,
    config: Config,
) where
    TMessage: IMessage,
{
    let mut stream_input = stream_input.take().unwrap();
    while let Some(msg) = stream_input.recv().await {
        match config.level {
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

#[derive(Clone)]
pub struct Config {
    /// Уровень логгирования
    pub level: Level,
}

pub fn create<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, cmp_logger);
    Box::new(cmp)
}
