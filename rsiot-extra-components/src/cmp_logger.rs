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
    debug!("cmp_logger started");
    let header = match config.header.as_str() {
        "" => "".to_string(),
        _ => format!("{}: ", config.header),
    };
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
            Level::TRACE => trace!("{}{:?}", header, msg),
            Level::DEBUG => debug!("{}{:?}", header, msg),
            Level::INFO => info!("{}{:?}", header, msg),
            Level::WARN => warn!("{}{:?}", header, msg),
            Level::ERROR => error!("{}{:?}", header, msg),
        }
        match &output {
            Some(stream) => stream.send(msg).await.unwrap(),
            None => (),
        }
    }
}

/// Настройки компонента логгирования
#[derive(Clone, Debug)]
pub struct Config {
    /// Уровень логгирования
    pub level: Level,
    /// Добавляется в начале каждого сообщения
    pub header: String,
}

pub fn create<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, cmp_logger);
    Box::new(cmp)
}
