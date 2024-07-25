use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::EventSeverity;
use super::State;

/// Область памяти output
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    /// Состояние события
    pub state: State,

    /// Статус для вывода на hmi
    pub hmi_status: QHmiStatus,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiStatus {
    pub state: State,
    pub text: String,
    pub event_severity: EventSeverity,
    pub id: Uuid,
}
