//! Компонент для логгирования сообщений

use async_trait::async_trait;
use tracing::{debug, error, info, trace, warn, Level};

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

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
    TMessage: MsgDataBound,
{
    async fn process(
        &self,
        config: Config,
        in_out: CmpInOut<TMessage>,
    ) -> Result<(), ComponentError> {
        process(
            config,
            in_out.clone_with_new_id("cmp_logger", AuthPermissions::FullAccess),
        )
        .await
    }
}

async fn process<TMessage>(
    config: Config,
    mut in_out: CmpInOut<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound,
{
    let header = match config.header.as_str() {
        "" => "".to_string(),
        _ => format!("{}: ", config.header),
    };
    while let Ok(msg) = in_out.recv_input().await {
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

/// Компонент cmp_logger
pub type Cmp<TMessage> = Component<Config, TMessage>;

// TODO - функция фильтрации fn_input
