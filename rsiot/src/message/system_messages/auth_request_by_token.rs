use serde::{Deserialize, Serialize};

/// Запрос авторизации по токену
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthRequestByToken {
    /// Токен
    pub token: String,
}
