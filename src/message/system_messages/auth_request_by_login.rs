use serde::{Deserialize, Serialize};

use crate::message::MsgKey;

/// Запрос авторизации по логину и паролю
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthRequestByLogin {
    /// Логин
    pub login: String,
    /// Пароль
    pub password: String,
}

impl MsgKey for AuthRequestByLogin {
    fn key(&self) -> String {
        "".to_string()
    }
}
