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
    pub fn_input: fn(Message<TMsg>) -> anyhow::Result<Option<String>>,
}
// ANCHOR: Config
