use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::message::{AuthPermissions, MsgKey};

/// Подтверждение авторизации
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponseOk {
    /// Сгенерированный токен
    pub token: String,

    /// Разрешения
    pub perm: AuthPermissions,

    /// Идентификаторы компонентов, через которые сообщение было получено
    pub trace_ids: HashSet<Uuid>,

    /// Логин для входа
    pub login: String,
}

impl MsgKey for AuthResponseOk {
    fn key(&self) -> String {
        "".to_string()
    }
}
