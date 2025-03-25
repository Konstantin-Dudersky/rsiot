//! Компонент для логгирования сообщений

use async_trait::async_trait;
pub use tracing::Level;
use tracing::{debug, error, info, trace, warn};

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, Message, MsgDataBound},
};

/// Настройки компонента логгирования
#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    /// Уровень логгирования
    pub level: Level,

    /// Функция преобразования входящих сообщений в записи.
    ///
    /// Можно реализовать фильтрацию сообщений.
    ///
    /// **Примеры**
    ///
    /// - Логгирование всех сообщений
    ///
    /// ```rust
    /// fn_input: |msg| Ok(Some(msg.serialize()?)),
    /// ```
    ///
    /// - Логгирование всех сообщений с заголовком
    ///
    /// ```rust
    /// fn_input: |msg| {
    ///     let text = msg.serialize()?;
    ///     let text = format!("Header: {text}");
    ///     Ok(Some(text))
    /// },
    /// ```
    pub fn_input: fn(Message<TMsg>) -> anyhow::Result<Option<String>>,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        process(
            config,
            in_out.clone_with_new_id("cmp_logger", AuthPermissions::FullAccess),
        )
        .await
    }
}

async fn process<TMsg>(
    config: Config<TMsg>,
    mut in_out: CmpInOut<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let text = (config.fn_input)(msg);
        // ошибка сериализации
        let Ok(text) = text else {
            warn!("Logger Error: {:?}", text);
            continue;
        };
        // фильтрация
        let Some(text) = text else { continue };
        match config.level {
            Level::TRACE => trace!("{text}"),
            Level::DEBUG => debug!("{text}"),
            Level::INFO => info!("{text}"),
            Level::WARN => warn!("{text}"),
            Level::ERROR => error!("{text}"),
        }
    }
    Ok(())
}

/// Компонент cmp_logger
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
