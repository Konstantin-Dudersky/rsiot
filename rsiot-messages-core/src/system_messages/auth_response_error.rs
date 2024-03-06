use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Ответ на запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponseErr {
    pub error: String,

    /// Идентификаторы компонентов, через которые сообщение было получено
    pub trace_ids: HashSet<Uuid>,
}
