//! Компонент для логгирования сообщений

use tracing::{debug, error, info, trace, warn, Level};

use rsiot_component_core::{Component, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

async fn cmp_logger<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config,
) where
    TMessage: IMessage,
{
    info!("cmp_logger stop started");
    let mut input = match input {
        Some(val) => val,
        None => {
            let msg = "Input stream is None";
            error!("{:?}", msg);
            return;
        }
    };
    while let Some(msg) = input.recv().await {
        match config.level {
            Level::TRACE => trace!("{:?}", msg),
            Level::DEBUG => debug!("{:?}", msg),
            Level::INFO => info!("{:?}", msg),
            Level::WARN => warn!("{:?}", msg),
            Level::ERROR => error!("{:?}", msg),
        }
        match &output {
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
