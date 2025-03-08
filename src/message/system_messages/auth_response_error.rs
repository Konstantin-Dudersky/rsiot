use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::message::MsgKey;

/// Отказ в авторизации
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponseErr {
    /// Причина отказа
    pub error: String,

    /// Идентификаторы компонентов, через которые сообщение было получено
    pub trace_ids: HashSet<Uuid>,
}

impl MsgKey for AuthResponseErr {
    fn key(&self) -> String {
        "".to_string()
    }
}
