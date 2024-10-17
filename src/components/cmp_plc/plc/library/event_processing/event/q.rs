use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::EventSeverity;
use super::State;

/// Область памяти output
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Q {
    /// Состояние события
    pub state: State,

    /// Статус для вывода на hmi
    pub hmi_status: QHmiStatus,
}

/// Состояние для HMI
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct QHmiStatus {
    /// Состояние события
    pub state: State,
    /// Текст события
    pub text: String,
    /// Уровень события
    pub event_severity: EventSeverity,
    /// Идентификатор события
    pub id: Uuid,
}
