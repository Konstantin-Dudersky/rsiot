use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::EventSeverity;

/// Входная структура
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    /// Идентификатор сообщения
    pub id: Uuid,

    /// Сигнал наступления события
    pub signal: bool,

    /// Уровень события
    pub event_severity: EventSeverity,

    /// Текст события
    pub text: String,

    /// Команда с hmi
    pub hmi_command: IHmiCommand,
}

/// Команда с hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IHmiCommand {
    /// Нет команды - по-умолчанию
    #[default]
    NoCommand,

    /// Квитирование сообщения из Hmi
    Ack(Uuid),

    /// Квитирование всех сообщений
    AckAll,
}
