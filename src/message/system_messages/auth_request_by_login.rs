use serde::{Deserialize, Serialize};

/// Запрос авторизации по логину и паролю
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthRequestByLogin {
    /// Логин
    pub login: String,
    /// Пароль
    pub password: String,
}
