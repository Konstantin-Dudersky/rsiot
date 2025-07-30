//! Компонент для логгирования сообщений

use async_trait::async_trait;
use tokio::task::JoinSet;
pub use tracing::Level;
use tracing::{debug, error, info, trace, warn};

use crate::{
    executor::{join_set_spawn, CmpInOut, Component, ComponentError, IComponentProcess},
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
    TMsg: 'static + MsgDataBound,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            in_out.clone_with_new_id("cmp_logger", AuthPermissions::FullAccess),
        )
        .await
    }
}

async fn fn_process<TMsg>(
    config: Config<TMsg>,
    in_out: CmpInOut<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set = JoinSet::new();

    let task = TaskLogger { config, in_out };
    join_set_spawn(&mut task_set, "cmp_logger", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res.unwrap()?;
    }

    Ok(())
}

struct TaskLogger<TMsg>
where
    TMsg: MsgDataBound,
{
    pub config: Config<TMsg>,
    pub in_out: CmpInOut<TMsg>,
}
impl<TMsg> TaskLogger<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), ComponentError> {
        while let Ok(msg) = self.in_out.recv_input().await {
            let text = (self.config.fn_input)(msg);
            // ошибка сериализации
            let Ok(text) = text else {
                warn!("Logger Error: {:?}", text);
                continue;
            };
            // фильтрация
            let Some(text) = text else { continue };
            match self.config.level {
                Level::TRACE => trace!("{text}"),
                Level::DEBUG => debug!("{text}"),
                Level::INFO => info!("{text}"),
                Level::WARN => warn!("{text}"),
                Level::ERROR => error!("{text}"),
            }
        }

        Ok(())
    }
}

/// Компонент cmp_logger
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
