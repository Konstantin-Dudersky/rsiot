use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::message::AuthPermissions;

/// Ответ на запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponseOk {
    /// Сгенерированный токен
    pub token: String,

    /// Разрешения
    pub perm: AuthPermissions,

    /// Идентификаторы компонентов, через которые сообщение было получено
    pub trace_ids: HashSet<Uuid>,

    pub login: String,
}
