use crate::message::Message;

use tracing::Level;

// ANCHOR: Config
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
// ANCHOR: Config
