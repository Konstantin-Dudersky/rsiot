use serde::{Deserialize, Serialize};

/// Запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthRequest {
    login: String,
    password: String,
}
