//! Компонент для логгирования сообщений

use async_trait::async_trait;
use tracing::{debug, error, info, trace, warn, Level};

use rsiot_component_core::{
    Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::IMessage;

/// Настройки компонента логгирования
#[derive(Clone, Debug)]
pub struct Config {
    /// Уровень логгирования
    pub level: Level,
    /// Добавляется в начале каждого сообщения
    pub header: String,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage> IComponentProcess<Config, TMessage> for Component<Config, TMessage>
where
    TMessage: IMessage,
{
    async fn process(
        &self,
        config: Config,
        input: CmpInput<TMessage>,
        output: CmpOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        process(config, input, output, cache).await
    }
}

async fn process<TMessage>(
    config: Config,
    mut input: CmpInput<TMessage>,
    _output: CmpOutput<TMessage>,
    _cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage,
{
    debug!("cmp_logger started");
    let header = match config.header.as_str() {
        "" => "".to_string(),
        _ => format!("{}: ", config.header),
    };
    while let Ok(msg) = input.recv().await {
        match config.level {
            Level::TRACE => trace!("{}{:?}", header, msg),
            Level::DEBUG => debug!("{}{:?}", header, msg),
            Level::INFO => info!("{}{:?}", header, msg),
            Level::WARN => warn!("{}{:?}", header, msg),
            Level::ERROR => error!("{}{:?}", header, msg),
        }
    }
    Ok(())
}

pub type Cmp<TMessage> = Component<Config, TMessage>;
